use ui::{
    app::{Application, ApplicationDelegate},
    message::Message,
    std::{flex::Row, label::Label, text_button::TextButton},
    ui_state::UIState,
    value::Value,
    window_request::WindowRequest,
};

pub struct AppDelegate;
impl ApplicationDelegate for AppDelegate {
    fn create_ui_state(&self) -> UIState {
        let mut state = UIState::new();
        state.register("label_text", "Hello World!");
        state
    }

    fn app_will_start(&self, app: &mut Application) {
        app.request_window(
            WindowRequest::new(480, 240)
                .with_title("Label Example")
                .with_ui(|_| {
                    Row::new(|| {
                        vec![
                            TextButton::new("Btn")
                                .on_click(|message_ctx| {
                                    message_ctx.dispatch(
                                        Message::new("set_text").with_arg("Label set by button"),
                                    )
                                })
                                .into(),
                            Label::new(Value::Binding("label_text".into())).into(),
                        ]
                    })
                    .into()
                }),
        );
    }

    fn handle_message(&mut self, mut message: Message, state: &mut UIState) {
        if message.target == "set_text" {
            state.set("label_text", message.args.remove(0));
        }
    }
}

fn main() {
    Application::start(AppDelegate {});
}