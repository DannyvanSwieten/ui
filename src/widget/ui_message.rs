use crate::{tree::ElementId, user_interface::value::Var};

pub struct UIMessage {
    pub sender: ElementId,
    pub receiver: ElementId,
    pub target: String,
    pub args: Vec<Var>,
}

impl UIMessage {
    pub fn new(sender: usize, receiver: usize, target: &str) -> Self {
        Self {
            sender,
            receiver,
            target: target.into(),
            args: Vec::new(),
        }
    }

    pub fn with_args<I>(mut self, args: I) -> Self
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Into<Var>,
    {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    pub fn with_arg(mut self, arg: impl Into<Var>) -> Self {
        self.args.push(arg.into());
        self
    }
}
