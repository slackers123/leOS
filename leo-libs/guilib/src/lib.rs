//! This gui layout system is largely inspired by clay and egui.
use std::{fmt::Debug, sync::Arc};

use corelib::types::Float;
use mathlib::vectors::Vec2F;
use widgets::button::Button;

pub mod base;
pub mod widgets;

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub pos: Option<Vec2F>,
    pub size: Vec2F,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct UiColor {
    pub r: Float,
    pub g: Float,
    pub b: Float,
    pub a: Float,
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

pub trait Widget<I>: Debug + Sized + Clone {
    fn container(&self) -> UiBox;
    fn produce(self, ctx: &mut UiContext, inner: I);
}

struct WidgetInt {
    ui_box: Arc<UiBox>, // made this an arc so maybe in the future we can only store unique UIBoxes
    shrink_cb: Option<Box<dyn FnMut(Float) -> Float + 'static>>,
}

/// This trait functions to convert Widgets into their internal representation.
/// It should usually not be implemented by library users. Instead implement the
/// [`Widget`] trait.
pub trait IntoWidgetInt<I>: Clone {
    const CAN_SHRINK: bool = false;
    #[allow(private_interfaces)]
    fn into_widget_int(self) -> WidgetInt;
    fn ui_box(&self) -> UiBox;
    fn produce(self, ctx: &mut UiContext, inner: I);
    fn shrink(&self, new_size: Float) -> Float;
}

impl<I, T: Widget<I> + 'static> IntoWidgetInt<I> for T {
    #[allow(private_interfaces)]
    fn into_widget_int(self) -> WidgetInt {
        let ui_box = self.container();
        WidgetInt {
            ui_box: Arc::new(ui_box),
            shrink_cb: T::CAN_SHRINK.then_some(Box::new(move |f| {
                let widget = self.clone();
                widget.shrink(f)
            })),
        }
    }

    fn ui_box(&self) -> UiBox {
        self.container()
    }
    fn produce(self, ctx: &mut UiContext, inner: I) {
        self.produce(ctx, inner);
    }
    fn shrink(&self, _new_size: Float) -> Float {
        0.0
    }
}

impl Debug for WidgetInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WidgetInt {{ ui_box: {:?}, shrinkable: {}}}",
            self.ui_box,
            self.shrink_cb.is_some()
        )
    }
}

#[derive(Debug, Clone)]
pub struct UiBox {
    pub sizing: Sizing,
    pub background: UiColor,
    pub layout_dir: LayoutDir,
    pub padding: Padding,
    pub child_gap: Float,
    pub min_width: Float,
    pub min_height: Float,
    pub max_width: Float,
    pub max_height: Float,
    pub valign: Align,
    pub halign: Align,
}

impl Default for UiBox {
    fn default() -> Self {
        Self {
            max_width: Float::INFINITY,
            max_height: Float::INFINITY,
            sizing: Sizing::default(),
            background: UiColor::default(),
            layout_dir: LayoutDir::default(),
            padding: Padding::default(),
            child_gap: f32::default(),
            min_width: f32::default(),
            min_height: f32::default(),
            valign: Align::default(),
            halign: Align::default(),
        }
    }
}

impl<I> Widget<I> for UiBox
where
    I: FnMut(&mut UiContext),
{
    fn container(&self) -> UiBox {
        self.clone()
    }

    fn produce(self, ctx: &mut UiContext, mut inner: I) {
        inner(ctx)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Align {
    #[default]
    Start,
    Center,
    End,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum SizeUnit {
    Fixed(Float),
    Grow,
    #[default]
    Fit,
}

impl SizeUnit {
    fn get_min(&self) -> Float {
        match self {
            Self::Fixed(s) => *s,
            _ => 0.0,
        }
    }
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

#[derive(Debug)]
pub struct FinalBox {
    pub pos: Vec2F,
    pub size: Rect,
    pub color: UiColor,
}

#[derive(Debug)]
pub struct FinishedLayoutStage {
    width: Float,
    height: Float,
    widget: WidgetInt,
    children: Vec<FinishedLayoutStage>,
}

impl FinishedLayoutStage {
    fn position(self, start: Vec2F) -> Vec<FinalBox> {
        let Self {
            width,
            height,
            widget,
            children,
        } = self;
        let ui_box = widget.ui_box;
        let mut boxes = vec![FinalBox {
            pos: start,
            size: Rect { width, height },
            color: ui_box.background,
        }];

        let total_child_gap = (children.len() as Float - 1.0).max(0.0) * ui_box.child_gap;

        let avail_width = height
            - ui_box.padding.horiz()
            - if ui_box.layout_dir == LayoutDir::LeftToRight {
                total_child_gap + children.iter().map(|c| c.width).sum::<Float>()
            } else {
                0.0
            };

        let avail_height = height
            - ui_box.padding.vert()
            - if ui_box.layout_dir == LayoutDir::TopToBottom {
                total_child_gap + children.iter().map(|c| c.height).sum::<Float>()
            } else {
                0.0
            };

        let mut stride = start + Vec2F::new(ui_box.padding.left, ui_box.padding.top);

        for child in children {
            let x_off = match ui_box.halign {
                Align::Start => 0.0,
                Align::Center => match ui_box.layout_dir {
                    LayoutDir::LeftToRight => avail_width / 2.0,
                    LayoutDir::TopToBottom => (avail_width - child.width) / 2.0,
                },
                Align::End => match ui_box.layout_dir {
                    LayoutDir::LeftToRight => avail_width,
                    LayoutDir::TopToBottom => avail_width - child.width,
                },
            };
            let y_off = match ui_box.valign {
                Align::Start => 0.0,
                Align::Center => match ui_box.layout_dir {
                    LayoutDir::LeftToRight => avail_height / 2.0,
                    LayoutDir::TopToBottom => (avail_height - child.height) / 2.0,
                },
                Align::End => match ui_box.layout_dir {
                    LayoutDir::LeftToRight => avail_height,
                    LayoutDir::TopToBottom => avail_height - child.height,
                },
            };

            let off = Vec2F::new(x_off, y_off);

            let add = Vec2F::new(child.width, child.height);
            boxes.append(&mut child.position(stride + off));

            match ui_box.layout_dir {
                LayoutDir::LeftToRight => {
                    stride.x += add.x + ui_box.child_gap;
                }
                LayoutDir::TopToBottom => {
                    stride.y += add.y + ui_box.child_gap;
                }
            }
        }

        boxes
    }
}

#[derive(Debug)]
pub struct GrowShrinkHeightStage {
    width: Float,
    height: Float,
    widget: WidgetInt,
    children: Vec<GrowShrinkHeightStage>,
}

impl GrowShrinkHeightStage {
    fn grow_shrink_height(self) -> FinishedLayoutStage {
        let Self {
            height,
            width,
            widget,
            mut children,
        } = self;

        let ui_box = widget.ui_box.clone();

        let mut remaining_height = self.height - ui_box.padding.horiz();

        if ui_box.layout_dir == LayoutDir::TopToBottom {
            remaining_height -= (ui_box.child_gap * (children.len() as Float - 1.0)).max(0.0);

            for child in children.iter() {
                remaining_height -= child.height;
            }
        }

        let shrink = remaining_height.is_sign_negative();

        let mut changable_children = children
            .iter_mut()
            .filter(|c| {
                c.widget.ui_box.sizing.height == SizeUnit::Grow
                    || shrink && c.widget.shrink_cb.is_some()
            })
            .collect::<Vec<_>>();

        while changable_children.len() > 0 && remaining_height.abs() > Float::EPSILON {
            let mut furthest = changable_children[0].height;
            let mut second_furthest = if shrink {
                Float::NEG_INFINITY
            } else {
                Float::INFINITY
            };
            let mut height_to_add = remaining_height;

            for child in changable_children.iter() {
                if (shrink && child.height > furthest) || (!shrink && child.height < furthest) {
                    second_furthest = furthest;
                    furthest = child.height;
                }
                if child.height == furthest {
                } else {
                    second_furthest = if shrink {
                        second_furthest.max(child.height)
                    } else {
                        second_furthest.min(child.height)
                    };
                    height_to_add = second_furthest - furthest;
                }
            }

            let rem_per_child = remaining_height / changable_children.len() as Float;
            height_to_add = if shrink {
                height_to_add.max(rem_per_child)
            } else {
                height_to_add.min(rem_per_child)
            };

            dbg!(rem_per_child);

            let mut to_rem = vec![];
            for (i, child) in changable_children.iter_mut().enumerate() {
                if (child.height == child.widget.ui_box.max_height && !shrink)
                    || (child.height == child.widget.ui_box.min_height && shrink)
                {
                    to_rem.push(i);
                    continue;
                }
                if child.height == furthest {
                    child.height += height_to_add;
                    remaining_height -= height_to_add;
                }
            }

            for rem in to_rem {
                changable_children.swap_remove(rem);
            }
        }

        FinishedLayoutStage {
            width,
            widget,
            height,
            children: children
                .into_iter()
                .map(|wf| wf.grow_shrink_height())
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct FitHeightStage {
    width: Float,
    height: Option<Float>,
    widget: WidgetInt,
    children: Vec<FitHeightStage>,
}

impl FitHeightStage {
    fn fit_height(self) -> GrowShrinkHeightStage {
        let Self {
            width,
            height,
            widget,
            children,
        } = self;

        let ui_box = widget.ui_box.clone();

        let children = children
            .into_iter()
            .map(|c| c.fit_height())
            .collect::<Vec<_>>();

        let height = if let Some(height) = height {
            height
        } else {
            match ui_box.sizing.height {
                SizeUnit::Fit => match ui_box.layout_dir {
                    LayoutDir::LeftToRight => {
                        ui_box.padding.vert()
                            + children
                                .iter()
                                .map(|c| c.height)
                                .max_by(|a, b| a.partial_cmp(b).expect("no NaNs in heights"))
                                .unwrap_or(0.0)
                    }
                    LayoutDir::TopToBottom => {
                        ui_box.padding.vert()
                            + children.iter().map(|fitted| fitted.height).sum::<f32>()
                    }
                },
                SizeUnit::Fixed(s) => s,
                SizeUnit::Grow => 0.0,
            }
        }
        .clamp(ui_box.min_height, ui_box.max_height);
        GrowShrinkHeightStage {
            width,
            height,
            widget,
            children,
        }
    }
}

#[derive(Debug)]
pub struct WrapStage {
    width: Float,
    widget: WidgetInt,
    children: Vec<WrapStage>,
}

impl WrapStage {
    pub fn wrap(self) -> FitHeightStage {
        let Self {
            width,
            mut widget,
            children,
        } = self;
        let children = children.into_iter().map(|c| c.wrap()).collect();

        FitHeightStage {
            children,
            height: (widget.shrink_cb.is_some() && width < widget.ui_box.sizing.width.get_min())
                .then(|| widget.shrink_cb.take().unwrap()(width)),
            width,
            widget,
        }
    }
}

#[derive(Debug)]
pub struct GrowShrinkWidthStage {
    width: Float,
    widget: WidgetInt,
    children: Vec<GrowShrinkWidthStage>,
}

impl GrowShrinkWidthStage {
    pub fn grow_shrink_width(self) -> WrapStage {
        let Self {
            width,
            widget,
            mut children,
        } = self;

        let ui_box = widget.ui_box.clone();

        let mut remaining_width = self.width - ui_box.padding.horiz();

        if ui_box.layout_dir == LayoutDir::LeftToRight {
            remaining_width -= (ui_box.child_gap * (children.len() as Float - 1.0)).max(0.0);

            for child in children.iter() {
                remaining_width -= child.width;
            }
        }

        let shrink = remaining_width.is_sign_negative();

        let mut changable_children = children
            .iter_mut()
            .filter(|c| {
                c.widget.ui_box.sizing.width == SizeUnit::Grow
                    || shrink && c.widget.shrink_cb.is_some()
            })
            .collect::<Vec<_>>();

        while changable_children.len() > 0 && remaining_width.abs() > Float::EPSILON {
            let mut furthest = changable_children[0].width;
            let mut second_furthest = if shrink {
                Float::NEG_INFINITY
            } else {
                Float::INFINITY
            };
            let mut width_to_add = remaining_width;

            for child in changable_children.iter() {
                if (shrink && child.width > furthest) || (!shrink && child.width < furthest) {
                    second_furthest = furthest;
                    furthest = child.width;
                }
                if child.width == furthest {
                } else {
                    second_furthest = if shrink {
                        second_furthest.max(child.width)
                    } else {
                        second_furthest.min(child.width)
                    };
                    width_to_add = second_furthest - furthest;
                }
            }

            let rem_per_child = remaining_width / changable_children.len() as Float;
            width_to_add = if shrink {
                width_to_add.max(rem_per_child)
            } else {
                width_to_add.min(rem_per_child)
            };

            let mut to_rem = vec![];
            for (i, child) in changable_children.iter_mut().enumerate() {
                if (child.width == child.widget.ui_box.max_width && !shrink)
                    || (child.width == child.widget.ui_box.min_width && shrink)
                {
                    to_rem.push(i);
                    continue;
                }
                if child.width == furthest {
                    child.width += width_to_add;
                    remaining_width -= width_to_add;
                }
            }

            for rem in to_rem {
                changable_children.swap_remove(rem);
            }
        }

        WrapStage {
            width,
            widget,
            children: children
                .into_iter()
                .map(|wf| wf.grow_shrink_width())
                .collect(),
        }
    }
}

pub struct UiContext {
    widget: WidgetInt,
    children: Vec<GrowShrinkWidthStage>,
}

impl UiContext {
    fn new<I>(widget: impl IntoWidgetInt<I>) -> Self {
        Self {
            children: vec![],
            widget: widget.into_widget_int(),
        }
    }

    fn root_container() -> Self {
        Self {
            children: vec![],
            widget: WidgetInt {
                ui_box: Arc::new(UiBox::default()),
                shrink_cb: None,
            },
        }
    }

    /// close the UiContext and perform Fit sizing
    fn fit_width(self) -> GrowShrinkWidthStage {
        let Self { children, widget } = self;

        let ui_box = widget.ui_box.clone();
        let width = match ui_box.sizing.width {
            SizeUnit::Fit => match ui_box.layout_dir {
                LayoutDir::LeftToRight => {
                    ui_box.padding.horiz()
                        + (children.len() as Float - 1.0).max(0.0) * ui_box.child_gap
                        + children.iter().map(|fitted| fitted.width).sum::<f32>()
                }
                LayoutDir::TopToBottom => {
                    ui_box.padding.horiz()
                        + children
                            .iter()
                            .map(|c| c.width)
                            .max_by(|a, b| a.partial_cmp(b).expect("no NaNs in widths"))
                            .unwrap_or(0.0)
                }
            },
            SizeUnit::Fixed(s) => s,
            SizeUnit::Grow => 0.0,
        }
        .clamp(ui_box.min_width, ui_box.max_width);

        GrowShrinkWidthStage {
            widget,
            width,
            children,
        }
    }

    /// Add a widget to the current UiContext.
    pub fn add<W, I>(&mut self, widget: W, inner: I)
    where
        W: IntoWidgetInt<I>,
    {
        let mut sub_ctx = UiContext::new(widget.clone());

        widget.produce(&mut sub_ctx, inner);

        self.children.push(sub_ctx.fit_width());
    }
}

pub fn gui_test() -> Vec<drawlib::stroking::Path> {
    use SizeUnit::*;
    let mut ui = UiContext::root_container();

    ui.add(
        UiBox {
            sizing: Sizing {
                width: Fixed(1600.0),
                height: Fit {},
            },
            background: UiColor::BLUE,
            layout_dir: LayoutDir::TopToBottom,
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
                |ui| {
                    for i in 0..10 {
                        ui.add(
                            UiBox {
                                child_gap: i as Float,
                                ..Default::default()
                            },
                            |_| {},
                        );
                    }
                },
            );

            ui.add(
                UiBox {
                    sizing: Sizing {
                        width: Grow,
                        height: Grow,
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

            ui.add(
                Button {
                    ..Default::default()
                },
                (),
            );
        },
    );

    let fw = ui.fit_width();

    let gsw = fw.grow_shrink_width();

    let wrapped = gsw.wrap();

    let fh = wrapped.fit_height();

    let gsh = fh.grow_shrink_height();

    let pos = gsh.position(Vec2F::ZERO);

    dbg!(pos);

    todo!("generate primitives")
}
