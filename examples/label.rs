use ui::{
    application::Application,
    application_delegate::ApplicationDelegate,
    message::Message,
    mutation::Mutation,
    ui_state::UIState,
    value::{Value, Var},
    widget::{Center, Label},
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
            WindowRequest::new(500, 500)
                .with_title("Label Example")
                .with_ui(|_| {
                    Box::new(Center::new(|| {
                        Box::new(Label::new(Value::Binding("hello".to_string())))
                    }))
                }),
        );
    }

    fn app_started(&self, app: &mut Application) {
        app.dispatch(Message::new("SET_LABEL_TEXT").with_args(&[Var::String("Set Label Text")]))
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
