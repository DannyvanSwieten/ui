use std::rc::Rc;

use crate::{
    canvas::{color::Color, paint::Paint},
    geo::{Point, Rect, Size},
    painter::Painter,
    user_interface::ui_state::UIState,
    widget::{constraints::BoxConstraints, style::Insets, BuildCtx, Child, SizeCtx, Widget},
};

pub struct ContainerPainter {
    color: Option<Color>,
}

impl Painter for ContainerPainter {
    fn paint(&self, paint_ctx: &crate::painter::PaintCtx, canvas: &mut dyn crate::canvas::Canvas) {
        if let Some(color) = &self.color {
            let paint = Paint::new(*color);
            canvas.draw_rect(
                &Rect::new_from_size(paint_ctx.local_bounds().size()),
                &paint,
            )
        }
    }
}

pub struct Container {
    width: Option<f32>,
    height: Option<f32>,
    child: Child,
    color: Option<Color>,
    padding: Option<Insets>,
}

impl Container {
    pub fn new<C>(child: C) -> Self
    where
        C: Fn(&UIState) -> Box<dyn Widget> + 'static,
    {
        Self {
            width: None,
            height: None,
            child: Rc::new(child),
            color: None,
            padding: None,
        }
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_padding(mut self, padding: Insets) -> Self {
        self.padding = Some(padding);
        self
    }
}

impl Widget for Container {
    fn build(&self, build_ctx: &mut BuildCtx) -> crate::widget::Children {
        vec![(self.child)(build_ctx.ui_state())]
    }

    fn painter(&self, _ui_state: &UIState) -> Option<Box<dyn Painter>> {
        Some(Box::new(ContainerPainter { color: self.color }))
    }

    fn calculate_size(
        &self,
        _children: &[usize],
        constraints: &BoxConstraints,
        _size_ctx: &SizeCtx,
    ) -> Option<Size> {
        let width = if let Some(width) = self.width {
            constraints.max_width().unwrap_or(std::f32::MAX).min(width)
        } else {
            constraints.max_width().unwrap()
        };

        let height = if let Some(height) = self.height {
            constraints
                .max_height()
                .unwrap_or(std::f32::MAX)
                .min(height)
        } else {
            constraints.max_height().unwrap()
        };

        Some(Size::new(width, height))
    }

    fn layout(
        &self,
        _ui_state: &UIState,
        layout_ctx: &mut crate::widget::LayoutCtx,
        size: Size,
        children: &[usize],
    ) {
        let padding = self.padding.unwrap_or_default();
        let child_size = layout_ctx.preferred_size(
            children[0],
            &BoxConstraints::new()
                .with_max_width(size.width - padding.left - padding.right)
                .with_max_height(size.height - padding.top - padding.bottom),
        );

        let child_size = if let Some(child_size) = child_size {
            Size::new(
                child_size.width.min(size.width),
                child_size.height.min(size.height),
            )
        } else {
            Size::new(
                size.width - padding.left - padding.right,
                size.height - padding.top - padding.bottom,
            )
        };

        let child_offset = Point::new(
            (size.width - child_size.width) / 2.0,
            (size.height - child_size.height) / 2.0,
        );

        let child_bounds = crate::geo::Rect::new(child_offset, child_size);
        layout_ctx.set_child_bounds(children[0], child_bounds);
    }
}
