use crate::app::{message::ApplicationMessage, Senders};

pub struct ApplicationCtx {
    senders: Senders,
}

impl ApplicationCtx {
    pub fn new(senders: Senders) -> Self {
        Self { senders }
    }

    pub fn send(&mut self, message: ApplicationMessage) {
        self.senders
            .application_message_queue()
            .send(message)
            .unwrap();
    }
}
