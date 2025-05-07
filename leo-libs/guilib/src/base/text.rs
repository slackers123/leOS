use std::sync::Arc;

use corelib::types::Float;

use crate::{IntoWidgetInt, SizeUnit, UiBox, WidgetInt};

#[derive(Debug, Default, Clone)]
pub struct Text {
    pub content: String,
    pub font_size: Float,
}

impl IntoWidgetInt<()> for Text {
    const CAN_SHRINK: bool = true;

    #[allow(private_interfaces)]
    fn into_widget_int(self) -> WidgetInt {
        let ui_box = self.ui_box();
        WidgetInt {
            ui_box: Arc::new(ui_box),
            shrink_cb: Some(Box::new(move |f| {
                let widget = self.clone();
                widget.shrink(f)
            })),
        }
    }

    fn ui_box(&self) -> crate::UiBox {
        let mut ui_box = UiBox::default();

        // FIXME: this is obviously not correct and just a dummy implementation

        ui_box.sizing.width = SizeUnit::Fixed(self.content.len() as Float * self.font_size);

        ui_box.sizing.height = SizeUnit::Fixed(self.font_size);

        ui_box
    }

    fn produce(self, _ctx: &mut crate::UiContext, _: ()) {}

    fn shrink(&self, new_width: Float) -> Float {
        let mut stride = 0.0;
        let mut line_cnt = 0;
        for word in self.content.split_whitespace() {
            let word_len = word.len() as Float * self.font_size;
            stride += word_len;
            if stride >= new_width {
                line_cnt += 1;
                stride = word_len;
            }
        }

        line_cnt as Float * self.font_size
    }
}
