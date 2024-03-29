use ui::{
    app::{message::ApplicationMessage, Application, ApplicationDelegate},
    std::{flex::Row, label::label_with_bind, text_button::text_button},
    user_interface::ui_state::UIState,
    window_request::WindowRequest,
};

pub struct AppDelegate;
impl ApplicationDelegate for AppDelegate {
    fn create_ui_state(&self) -> UIState {
        let mut state = UIState::new();
        state.register("counter_value", 0);
        state
    }

    fn app_will_start(&self, app: &mut Application) {
        app.request_window(
            WindowRequest::new(480, 240)
                .with_title("Label Example")
                .with_ui(|_| {
                    Row::new(|| {
                        vec![
                            text_button("Count", |message_ctx| {
                                message_ctx.send(ApplicationMessage::new("count"))
                            }),
                            text_button("Reset", |message_ctx| {
                                message_ctx.send(ApplicationMessage::new("reset"))
                            }),
                            label_with_bind("counter_value"),
                        ]
                    })
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
