//! This gui layout system is largely inspired by clay and egui.
use std::fmt::Debug;

use corelib::types::Float;
use mathlib::vectors::Vec2F;

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub pos: Option<Vec2F>,
    pub size: Vec2F,
}

#[derive(Debug, Clone, Copy)]
pub struct UiColor {
    pub r: Float,
    pub g: Float,
    pub b: Float,
    pub a: Float,
}

impl Default for UiColor {
    fn default() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }
}

impl UiColor {
    pub const PINK: UiColor = UiColor {
        r: 1.0,
        g: 0.7529411765,
        b: 0.7960784314,
        a: 1.0,
    };

    pub const BLUE: UiColor = UiColor {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };

    pub const LIGHT_BLUE: UiColor = UiColor {
        r: 0.678,
        g: 0.847,
        b: 0.902,
        a: 1.0,
    };

    pub const YELLOW: UiColor = UiColor {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
}

pub trait Widget {
    fn ui_box(&self) -> UiBox;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct UiBox {
    pub sizing: Sizing,
    pub background: UiColor,
    pub layout_dir: LayoutDir,
    pub padding: Padding,
    pub child_gap: Float,
}

impl Widget for UiBox {
    fn ui_box(&self) -> UiBox {
        *self
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum SizeUnit {
    Fixed(Float),
    Grow,
    #[default]
    Fit,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Sizing {
    width: SizeUnit,
    height: SizeUnit,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum LayoutDir {
    #[default]
    LeftToRight,
    TopToBottom,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Padding {
    left: Float,
    right: Float,
    top: Float,
    bottom: Float,
}

impl Padding {
    pub fn horiz(&self) -> Float {
        self.left + self.right
    }

    pub fn vert(&self) -> Float {
        self.top + self.bottom
    }
}

#[derive(Debug, Default)]
pub struct Rect {
    pub width: Float,
    pub height: Float,
}

pub struct LayoutContext {
    size: Rect,
    ui_box: UiBox,
    children: Vec<LayoutContext>,
}

impl LayoutContext {
    /// close the layout context and perform grow and shrink sizing as well as final positioning
    pub fn grow_shrink_pos(mut self, start: Vec2F) -> Vec<(Vec2F, Rect)> {
        let mut remaining_width = self.size.width
            - (self.ui_box.padding.horiz()
                + (self.ui_box.child_gap * (self.children.len() as Float - 1.0)).max(0.0));

        for child in self.children.iter() {
            remaining_width -= child.size.width;
        }

        let mut growable_children = self
            .children
            .iter_mut()
            .filter(|c| c.ui_box.sizing.width == SizeUnit::Grow)
            .collect::<Vec<_>>();

        while growable_children.len() > 0 && remaining_width > 0.0 {
            let mut smallest = growable_children[0].size.width;
            let mut second_smallest = Float::INFINITY;
            let mut width_to_add = remaining_width;

            for child in growable_children.iter() {
                if child.size.width < smallest {
                    second_smallest = smallest;
                    smallest = child.size.width;
                }
                if child.size.width > smallest {
                    second_smallest = second_smallest.min(child.size.width);
                    width_to_add = second_smallest - smallest;
                }
            }

            width_to_add = width_to_add.min(remaining_width / growable_children.len() as Float);

            for child in growable_children.iter_mut() {
                if child.size.width == smallest {
                    child.size.width += width_to_add;
                    remaining_width -= width_to_add;
                }
            }
        }

        let mut boxes = vec![(start, self.size)];

        let mut stride = self.ui_box.padding.left;
        for child in self.children {
            let total_add = child.size.width + self.ui_box.child_gap;

            boxes.append(
                &mut child.grow_shrink_pos(start + Vec2F::new(stride, self.ui_box.padding.top)),
            );

            stride += total_add;
        }

        boxes
    }
}

impl Debug for LayoutContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UiContext {{\nsize: {:?}, \nchildren: {:?}\n}}",
            self.size, self.children
        )?;
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct UiContext {
    ui_box: UiBox,
    children: Vec<LayoutContext>,
}

impl UiContext {
    pub fn new(ui_box: UiBox) -> Self {
        Self {
            ui_box,
            children: vec![],
        }
    }

    /// close the UiContext and perform Fit sizing
    fn close_ctx(self) -> LayoutContext {
        let mut size = Rect::default();
        match self.ui_box.layout_dir {
            LayoutDir::LeftToRight => {
                size.width = match self.ui_box.sizing.width {
                    SizeUnit::Fit => {
                        // add padding and child gap
                        self.ui_box.padding.horiz()
                            + (self.children.len() as Float - 1.0).max(0.0) * self.ui_box.child_gap

                        // add child sizes
                        + self.children.iter().map(|ctx| ctx.size.width).sum::<f32>()
                    }
                    SizeUnit::Fixed(s) => s,
                    SizeUnit::Grow => 0.0,
                };

                size.height = match self.ui_box.sizing.height {
                    SizeUnit::Fit => {
                        // add padding
                        self.ui_box.padding.vert()

                        // add largest child size
                        + self.children.iter().map(|ctx| ctx.size.height)
                            .max_by(|a, b| a.partial_cmp(b).expect("no NaNs in heights")).unwrap_or(0.0)
                    }
                    SizeUnit::Fixed(s) => s,
                    SizeUnit::Grow => 0.0,
                };
            }
            _ => todo!("top-to-bottom sizing"),
        }

        LayoutContext {
            size,
            ui_box: self.ui_box,
            children: self.children,
        }
    }

    pub fn add<F>(&mut self, widget: impl Widget, inner: F)
    where
        F: FnOnce(&mut UiContext),
    {
        let mut sub_ctx = UiContext::new(widget.ui_box());

        inner(&mut sub_ctx);

        self.children.push(sub_ctx.close_ctx());
    }
}

pub fn gui_test() {
    use SizeUnit::*;
    let mut ui = UiContext::new(UiBox::default());

    ui.add(
        UiBox {
            sizing: Sizing {
                width: Fixed(1600.0),
                height: Fit,
            },
            background: UiColor::BLUE,
            ..Default::default()
        },
        |ui| {
            ui.add(
                UiBox {
                    sizing: Sizing {
                        width: Fixed(300.0),
                        height: Fixed(300.0),
                    },
                    background: UiColor::PINK,
                    ..Default::default()
                },
                |_| {},
            );

            ui.add(
                UiBox {
                    sizing: Sizing {
                        width: Grow,
                        height: Fixed(200.0),
                    },
                    background: UiColor::YELLOW,
                    ..Default::default()
                },
                |_| {},
            );

            ui.add(
                UiBox {
                    sizing: Sizing {
                        width: Fixed(300.0),
                        height: Fixed(300.0),
                    },
                    background: UiColor::LIGHT_BLUE,
                    ..Default::default()
                },
                |_| {},
            );
        },
    );

    let layout = ui.close_ctx();

    println!("{layout:?}");

    let boxes = layout.grow_shrink_pos(Vec2F::ZERO);

    println!("{boxes:?}");
}
