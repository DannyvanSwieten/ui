use std::time::Duration;

use ui::{
    app::{message::ApplicationMessage, Application, ApplicationDelegate},
    canvas::color::Color32f,
    std::animated_color_box::AnimatedColorBox,
    user_interface::ui_state::UIState,
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
                .with_title("Animated Color Box")
                .with_ui(|_| {
                    AnimatedColorBox::new(
                        vec![
                            Color32f::new_grey(0.0),
                            Color32f::new_rgb(1.0, 0.0, 0.0),
                            Color32f::new_rgb(0.0, 1.0, 0.0),
                            Color32f::new_rgb(0.0, 0.0, 1.0),
                            Color32f::new_rgb(1.0, 1.0, 1.0),
                        ],
                        Duration::new(4, 0),
                    )
                    .into()
                }),
        );
    }

    fn handle_message(&mut self, _: ApplicationMessage, _: &mut UIState) {}
}

fn main() {
    Application::start(AppDelegate {});
}
