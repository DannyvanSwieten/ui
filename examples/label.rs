use ui::{
    application::Application,
    application_delegate::ApplicationDelegate,
    message::Message,
    mutation::Mutation,
    ui_state::UIState,
    value::{Value, Var},
    widget::{Label, Row, TextButton},
    window_request::WindowRequest,
};

pub struct AppDelegate;
impl ApplicationDelegate for AppDelegate {
    fn create_ui_state(&self) -> UIState {
        let mut state = UIState::new();
        state.register("hello", Var::String("Hello World!"));
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
                                    "SET_LABEL_TEXT",
                                    vec![Var::String("Label set by button")],
                                )
                            })),
                            Box::new(Label::new(Value::Binding("hello".into()))),
                        ])
                    }))
                }),
        );
    }

    fn handle_message(&mut self, message: Message, _: &UIState) -> Option<Mutation> {
        if message.target == "SET_LABEL_TEXT" {
            Some(Mutation {
                name: "hello".to_string(),
                value: message.args[0],
            })
        } else {
            None
        }
    }
}

fn main() {
    Application::start(AppDelegate {});
}
