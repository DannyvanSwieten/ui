use ui::{
    app::{message::ApplicationMessage, Application, ApplicationDelegate},
    canvas::color::Color32f,
    std::{container::Container, label::Label, list::List, viewport::Scrollable},
    user_interface::ui_state::UIState,
    widget::style::Insets,
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
                .with_title("Scroll Example")
                .with_ui(|_| {
                    Scrollable::new(|_| {
                        List::new(|| {
                            (0..15)
                                .map(|i| {
                                    let color = if i % 2 == 0 { 0.25 } else { 0.2 };
                                    Container::new(move |_| {
                                        Label::new(format!("Item {}", i)).into()
                                    })
                                    .with_height(50.0)
                                    .with_color(Color32f::new_grey(color).into())
                                    .into()
                                })
                                .collect()
                        })
                        .into()
                    })
                    .into()
                }),
        );
    }

    fn handle_message(&mut self, _message: ApplicationMessage, _state: &mut UIState) {}
}

fn main() {
    Application::start(AppDelegate {});
}
