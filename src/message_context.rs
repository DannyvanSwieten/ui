use crate::{message::Message, value::Var};

#[derive(Default)]
pub struct MessageCtx {
    messages: Vec<Message>,
}

impl MessageCtx {
    pub fn dispatch(&mut self, target: &str, args: Vec<Var>) {
        self.messages.push(Message {
            target: target.into(),
            args,
        })
    }

    pub fn messages(self) -> Vec<Message> {
        self.messages
    }
}
