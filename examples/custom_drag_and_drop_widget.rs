use ui::{
    application::Application,
    application_delegate::ApplicationDelegate,
    message::Message,
    ui_state::UIState,
    widget::{
        center::Center,
        drag_source::{DragSource, DragSourceData, DragSourceItem, DragSourceWidget},
        text_button::TextButton,
    },
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
                .with_title("Drag and Drop with custom widget Example")
                .with_ui(|_| {
                    Center::new(|| {
                        DragSource::new(|| TextButton::new("Button").into())
                            .with_drag_start(|| {
                                DragSourceData::new().with_item(DragSourceItem::new(
                                    DragSourceWidget::Widget(
                                        TextButton::new("You're Dragging Me").into(),
                                    ),
                                ))
                            })
                            .into()
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
