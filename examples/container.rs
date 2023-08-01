use ui::{
    app::{message::ApplicationMessage, Application, ApplicationDelegate},
    canvas::color::{Color, Color32f},
    std::{container::Container, flex::Row, label::label_with_bind, text_button::text_button},
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
                .with_title("Container Example")
                .with_ui(|_| {
                    Container::new(|_| text_button("Button in Container", |_| {}))
                        .with_height(35.0)
                        .with_color(Color32f::new_rgb(1.0, 0.5, 0.2).into())
                        .into()
                }),
        );
    }

    fn handle_message(&mut self, message: ApplicationMessage, state: &mut UIState) {
        if message.target == "count" {
            if let Some(old) = state.get("counter_value") {
                state.set("counter_value", old.as_integer().unwrap() + 1);
            }
        } else if message.target == "reset" {
            state.set("counter_value", 0);
        }
    }
}

fn main() {
    Application::start(AppDelegate {});
}
