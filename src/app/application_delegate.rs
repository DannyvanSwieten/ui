use super::Application;
use crate::{message::Message, ui_state::UIState};

pub trait ApplicationDelegate {
    fn create_ui_state(&self) -> UIState;
    fn app_will_start(&self, _app: &mut Application) {}
    fn app_will_quit(&mut self) {}
    fn app_started(&self, _app: &mut Application) {}
    fn handle_message(&mut self, message: Message, state: &mut UIState);

    fn quit_when_last_window_closes(&self) -> bool {
        true
    }
}
