use crate::{application::Application, message::Message, mutation::Mutation, ui_state::UIState};

pub trait ApplicationDelegate {
    fn create_ui_state(&self) -> UIState;
    fn app_will_start(&self, _app: &mut Application) {}
    fn app_started(&self, _app: &mut Application) {}
    fn handle_message(&mut self, message: Message, state: &UIState) -> Option<Mutation>;
}
