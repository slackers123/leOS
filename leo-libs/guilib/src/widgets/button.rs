use crate::{Align, Sizing, UiBox, Widget, base::text::Text};

/// A simple button
#[derive(Debug, Default, Clone)]
pub struct Button {
    pub sizing: Sizing,
    pub halign: Align,
    pub valign: Align,
    pub content: String,
}

impl Widget<()> for Button {
    fn container(&self) -> UiBox {
        UiBox {
            sizing: self.sizing,
            halign: self.halign,
            valign: self.valign,
            ..Default::default()
        }
    }
    fn produce(self, ctx: &mut crate::UiContext, _: ()) {
        ctx.add(
            Text {
                content: self.content,
                ..Default::default()
            },
            (),
        );
    }
}
