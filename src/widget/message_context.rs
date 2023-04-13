use crate::app::message::Message;

#[derive(Default)]
pub struct MessageCtx {
    messages: Vec<Message>,
}

impl MessageCtx {
    pub fn dispatch(&mut self, message: Message) {
        self.messages.push(message)
    }

    pub fn messages(self) -> Vec<Message> {
        self.messages
    }
}
