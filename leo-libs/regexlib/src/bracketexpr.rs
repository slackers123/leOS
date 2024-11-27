use crate::{
    parser::handle_special_chars,
    state_machine::{BracketExpr, InnerBrackExpr},
};

pub fn parse_bracket_expr(src: &[char], idx: &mut usize) -> BracketExpr {
    assert!(src[*idx] == '[');
    let mut res = BracketExpr {
        inverted: false,
        inner_be: vec![],
    };

    *idx += 1;
    if src[*idx] == '^' {
        res.inverted = true;
        *idx += 1;
    }

    while src[*idx] != ']' {
        let cur = src[*idx];

        if cur == '\\' {
            handle_special_chars(src[*idx + 1]);
            *idx += 2;
            continue;
        }

        *idx += 1;
        if src[*idx] == '-' && *idx + 1 < src.len() && src[*idx + 1] != ']' {
            *idx += 1;
            res.inner_be.push(InnerBrackExpr::Range(cur, src[*idx]));
            *idx += 1;
        } else {
            res.inner_be.push(InnerBrackExpr::Char(cur));
        }
    }
    res
}
