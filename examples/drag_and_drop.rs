use ui::{
    app::{message::Message, Application, ApplicationDelegate},
    std::{
        drag_source::DragSource, drop_target::DropTarget, flex::Row,
        text_button::TextButton,
    },
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
                .with_title("Drag and Drop Example")
                .with_ui(|_| {
                    Row::new(|| {
                        vec![
                            DragSource::<String>::new(|| TextButton::new("Child not source").into())
                                .with_child_when_dragging(|| {
                                    TextButton::new("Child when dragging").into()
                                })
                                .with_dragging_child(|| TextButton::new("Dragged Widget").into())
                                .with_drag_start(|| "Drag Data".to_string())
                                .into(),
                            DropTarget::<String>::new(|| TextButton::new("Drop Target").into())
                                .with_child_on_accept(|| TextButton::new("Child on accept").into())
                                .with_accept(|data| data == "Drag Data")
                                .into(),
                        ]
                    })
                    .into()
                }),
        );
    }

    fn handle_message(&mut self, mut _message: Message, _state: &mut UIState) {}
}

fn main() {
    Application::start(AppDelegate {});
}
