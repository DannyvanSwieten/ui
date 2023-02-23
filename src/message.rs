use crate::value::Var;

pub struct Message {
    pub target: String,
    pub args: Vec<Var>,
}

impl Message {
    pub fn new(target: &str) -> Self {
        Self {
            target: target.into(),
            args: Vec::new(),
        }
    }

    pub fn with_args(mut self, args: &[Var]) -> Self {
        self.args = args.into();
        self
    }
}
