use crate::user_interface::ui_state::UIState;

use super::{message::ApplicationMessage, Application};

pub trait ApplicationDelegate {
    fn create_ui_state(&self) -> UIState;
    fn app_will_start(&self, _app: &mut Application) {}
    fn app_will_quit(&mut self) {}
    fn app_started(&self, _app: &mut Application) {}
    fn handle_message(&mut self, message: ApplicationMessage, state: &mut UIState);

    fn quit_when_last_window_closes(&self) -> bool {
        true
    }
}
