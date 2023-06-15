use std::time::Duration;

use crate::{
    animation::animation_ctx::AnimationCtx,
    canvas::{
        color::{Color32f, Lerp},
        paint::Paint,
        Canvas,
    },
    geo::Size,
    painter::{render_ctx::RenderCtx, PaintCtx, Painter},
    user_interface::ui_state::UIState,
    widget::{constraints::BoxConstraints, BuildCtx, Child, Children, SizeCtx, Widget},
};

pub struct AnimatedColorBox {
    pub colors: Vec<Color32f>,
    child: Option<Child>,
    duration: Duration,
}

impl AnimatedColorBox {
    pub fn new(colors: Vec<Color32f>, duration: Duration) -> Self {
        Self {
            colors,
            child: None,
            duration,
        }
    }

    pub fn with_child(mut self, child: Child) -> Self {
        self.child = Some(child);
        self
    }
}

impl Widget for AnimatedColorBox {
    fn calculate_size(
        &self,
        _: &[usize],
        constraints: &BoxConstraints,
        _: &SizeCtx,
    ) -> Option<Size> {
        // Something, Somewhere, went terribly wrong
        // assert_eq!(1, children.len());

        // Return all the space that is given to this widget.
        Some(Size::new(
            constraints.max_width().unwrap(),
            constraints.max_height().unwrap(),
        ))
    }

    fn build(&self, _: &mut BuildCtx) -> Children {
        let mut children = Vec::new();
        if let Some(child) = &self.child {
            children.push((*child)())
        }

        children
    }

    fn painter(&self, _: &UIState) -> Option<Box<dyn Painter>> {
        Some(Box::new(AnimatedColorBoxPainter {
            colors: self.colors.clone(),
            color: self.colors[0],
            duration: self.duration,
        }))
    }
}

pub struct AnimatedColorBoxPainter {
    pub colors: Vec<Color32f>,
    pub color: Color32f,
    pub duration: Duration,
}

impl Painter for AnimatedColorBoxPainter {
    fn mounted(&self, render_ctx: &mut RenderCtx) {
        render_ctx.animation_request(0, self.duration)
    }

    fn animation_event(&mut self, ctx: &mut AnimationCtx) {
        match ctx.event() {
            crate::animation::animation_event::AnimationEvent::Start(_) => (),
            crate::animation::animation_event::AnimationEvent::Update(_, phase) => {
                let f = *phase * (self.colors.len() - 1) as f64;
                let index = f as usize;
                let next = index + 1;
                let fract = f - index as f64;
                self.color = self.colors[index].lerp(self.colors[next], fract as f32)
            }
            crate::animation::animation_event::AnimationEvent::End(_) => (),
        }
    }

    fn paint(&self, paint_ctx: &PaintCtx, canvas: &mut dyn Canvas) {
        canvas.draw_rect(paint_ctx.local_bounds(), &Paint::new(self.color))
    }
}
