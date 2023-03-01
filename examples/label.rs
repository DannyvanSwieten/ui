use ui::{
    application::Application,
    application_delegate::ApplicationDelegate,
    message::Message,
    ui_state::UIState,
    value::{Value, Var},
    widget::{flex::Row, label::Label, text_button::TextButton},
    window_request::WindowRequest,
};

pub struct AppDelegate;
impl ApplicationDelegate for AppDelegate {
    fn create_ui_state(&self) -> UIState {
        let mut state = UIState::new();
        state.register("hello", Var::StringLiteral("Hello World!"));
        state
    }

    fn app_will_start(&self, app: &mut Application) {
        app.request_window(
            WindowRequest::new(480, 240)
                .with_title("Label Example")
                .with_ui(|_| {
                    Box::new(Row::new(|| {
                        Some(vec![
                            Box::new(TextButton::new("Btn").on_click(|message_ctx| {
                                message_ctx.dispatch(
                                    Message::new("set_text")
                                        .with_string_literal("Label set by button"),
                                )
                            })),
                            Box::new(Label::new(Value::Binding("hello".into()))),
                        ])
                    }))
                }),
        );
    }

    fn handle_message(&mut self, mut message: Message, state: &mut UIState) {
        if message.target == "set_text" {
            state.set("hello", message.args.remove(0));
        }
    }
}

fn main() {
    Application::start(AppDelegate {});
}
