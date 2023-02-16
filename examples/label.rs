use ui::{
    value::{Value, Var},
    widget::{Application, ApplicationDelegate, Label, Message, Mutation, State, WindowRequest},
};

pub struct AppDelegate;
impl ApplicationDelegate for AppDelegate {
    fn create_state(&self) -> State {
        let mut state = State::new();
        state.register("hello", Var::String("Hello World!"));
        state
    }

    fn app_will_start(&self, app: &mut Application) {
        app.request_window(
            WindowRequest::new(500, 500)
                .with_ui(|state| Box::new(Label::new(Value::Binding("hello".to_string())))),
        );
    }

    fn app_started(&self, app: &mut Application) {
        println!("Application started")
    }

    fn handle_message(&mut self, message: Message, state: &State) -> Option<Mutation> {
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
