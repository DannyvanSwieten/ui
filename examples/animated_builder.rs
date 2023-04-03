use std::time::Duration;

use ui::{
    app::{Application, ApplicationDelegate},
    geo::Size,
    message::Message,
    std::{animated_builder::AnimatedBuilder, flex::Row, label::Label, sized_box::SizedBox},
    ui_state::UIState,
    value::Value,
    window_request::WindowRequest,
};

pub struct AppDelegate;
impl ApplicationDelegate for AppDelegate {
    fn create_ui_state(&self) -> UIState {
        UIState::new()
    }

    fn app_will_start(&self, app: &mut Application) {
        app.request_window(
            WindowRequest::new(480, 240)
                .with_title("Animated Builder Example")
                .with_ui(|_| {
                    Row::new(|| {
                        vec![
                            Label::new(Value::Const("Label 1".into())).into(),
                            AnimatedBuilder::new(Duration::new(4, 0), |_, phase| {
                                let width = 100.0 + 100.0 * phase;
                                vec![SizedBox::new(Size::new(width as f32, 100.0)).into()]
                            })
                            .into(),
                            Label::new(Value::Const("Label 2".into())).into(),
                        ]
                    })
                    .into()
                }),
        );
    }

    fn handle_message(&mut self, _: Message, _: &mut UIState) {}
}

fn main() {
    Application::start(AppDelegate {});
}
