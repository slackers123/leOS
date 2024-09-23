use corelib::reader::Reader;
use tabledir::TableDirectory;
use tables::{
    cmap::{self, CMAPSubtable},
    glyf::{self, GlyphTable},
    head,
    loca::{self, LocaTable},
    maxp,
};

mod tabledir;
pub mod tables;
mod util;

pub fn load_ttf<'a>(src: &'a [u8]) -> Font<'a> {
    let mut reader = Reader::new_big_endian(src, 0);

    let table_dirs = reader.read::<TableDirectory>();

    let cmap_table_record = table_dirs
        .table_records
        .iter()
        .find(|tr| tr.table_tag.0 == "cmap")
        .unwrap();

    let glyf_table_record = table_dirs
        .table_records
        .iter()
        .find(|tr| tr.table_tag.0 == "glyf")
        .unwrap();

    let loca_table_record = table_dirs
        .table_records
        .iter()
        .find(|tr| tr.table_tag.0 == "loca")
        .unwrap();

    let head_table_record = table_dirs
        .table_records
        .iter()
        .find(|tr| tr.table_tag.0 == "head")
        .unwrap();

    let maxp_table_record = table_dirs
        .table_records
        .iter()
        .find(|tr| tr.table_tag.0 == "maxp")
        .unwrap();

    let cmap = cmap::get_cmap(&mut reader, cmap_table_record);

    reader.set_pos(head_table_record.offset as usize);
    let head = reader.read::<head::HeadHeader>();

    reader.set_pos(maxp_table_record.offset as usize);
    let maxp = reader.read::<maxp::MaxpHeader>();

    reader.set_pos(loca_table_record.offset as usize);
    let loca = loca::get_loca_table(
        &mut reader,
        maxp.get_num_glyphs() as usize,
        head.index_to_loc_format,
    );

    Font {
        src,
        glyf_table_record_offset: glyf_table_record.offset as usize,
        loca,
        cmap,
    }
}

pub struct Font<'a> {
    src: &'a [u8],
    glyf_table_record_offset: usize,
    loca: LocaTable,
    cmap: CMAPSubtable,
}

impl<'a> Font<'a> {
    pub fn get_glyph(&self, c: char) -> GlyphTable {
        glyf::get_glyf(
            self.glyf_table_record_offset,
            &self.src,
            c,
            &self.loca,
            &self.cmap,
        )
    }
}
