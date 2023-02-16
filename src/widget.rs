use std::collections::hash_map::HashMap;

use crate::value::{Value, Var};

pub trait ApplicationDelegate {
    fn create_state(&self) -> State;
    fn app_will_start(&self, app: &mut Application);
    fn app_started(&self, app: &mut Application);
    fn handle_message(&mut self, message: Message, state: &State) -> Option<Mutation>;
}

type UIBuilder = dyn Fn(&State) -> Box<dyn Widget>;

pub struct WindowRequest {
    width: u32,
    height: u32,
    title: Option<String>,
    builder: Option<Box<UIBuilder>>,
}

impl WindowRequest {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            title: None,
            builder: None,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn with_ui<F: 'static>(mut self, builder: F) -> Self
    where
        F: Fn(&State) -> Box<dyn Widget>,
    {
        self.builder = Some(Box::new(builder));
        self
    }
}

pub struct Mutation {
    pub name: String,
    pub value: Var,
}

pub struct Message {
    pub target: String,
    pub args: Vec<Var>,
}

pub struct Application {
    state: State,
    ui: Document,
    window_requests: Vec<WindowRequest>,
    pending_messages: Vec<Message>,
}

impl Application {
    pub fn start(delegate: impl ApplicationDelegate + 'static) {
        let state = delegate.create_state();
        let ui = Document::new();
        let app = Self {
            state,
            ui,
            window_requests: Vec::new(),
            pending_messages: Vec::new(),
        };
        app.run(delegate);
    }

    fn run(mut self, delegate: impl ApplicationDelegate + 'static) {
        delegate.app_will_start(&mut self);
        loop {
            while let Some(request) = self.window_requests.first() {
                if let Some(builder) = &request.builder {
                    (*builder)(&mut self.state);
                }
            }
        }
    }

    pub fn build_ui(&mut self) {
        let build_ctx = self.ui.build(&self.state);
        self.state.bind(build_ctx.bindings)
    }

    pub fn request_window(&mut self, request: WindowRequest) {
        self.window_requests.push(request)
    }

    pub fn dispatch(&mut self, message: Message) {
        self.pending_messages.push(message)
    }
}

pub struct Element {
    widget: Box<dyn Widget>,
}

impl Element {
    pub fn new<W: Widget + 'static>(widget: W) -> Self {
        Self {
            widget: Box::new(widget),
        }
    }
}

struct Document {
    next_id: usize,
    elements: HashMap<usize, Element>,
}

impl Document {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            elements: HashMap::new(),
        }
    }

    fn next_id(&mut self) -> usize {
        self.next_id + 1
    }

    pub fn build(&mut self, state: &State) -> BuildCtx {
        let mut build_ctx = BuildCtx {
            id: 0,
            bindings: HashMap::new(),
        };

        for (_, element) in self.elements.iter_mut() {
            element.widget.build(&mut build_ctx, state)
        }

        build_ctx
    }
}

pub struct State {
    values: HashMap<String, Var>,
    dependees: HashMap<String, Vec<usize>>,
}

impl State {
    pub fn new() -> State {
        Self {
            values: HashMap::new(),
            dependees: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, default_value: Var) {
        self.values.insert(name.to_string(), default_value);
    }

    pub fn set(&mut self, name: &str, value: Var) -> Option<&Vec<usize>> {
        self.values.insert(name.to_string(), value);
        self.dependees.get(name)
    }

    pub fn get(&self, name: &str) -> Option<&Var> {
        self.values.get(name)
    }

    pub fn bind(&mut self, bindings: HashMap<String, Vec<usize>>) {
        for (name, bindings) in bindings {
            if !self.dependees.contains_key(&name) {
                self.dependees.insert(name.to_string(), bindings);
            } else {
                self.dependees.get_mut(&name).unwrap().extend(bindings);
            }
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BuildCtx {
    id: usize,
    bindings: HashMap<String, Vec<usize>>,
}

impl BuildCtx {
    pub fn bind(&mut self, name: &str) {
        if !self.bindings.contains_key(name) {
            self.bindings.insert(name.to_string(), Vec::new());
        }
        self.bindings.get_mut(name).unwrap().push(self.id);
    }
}

pub trait Widget {
    fn build(&mut self, build_ctx: &mut BuildCtx, state: &State);
}

pub struct Label {
    text: String,
    binding: Option<String>,
}

impl Label {
    pub fn new(default: Value) -> Self {
        match default {
            Value::Binding(binding) => Self {
                text: "".to_string(),
                binding: Some(binding),
            },
            Value::Const(c) => match c {
                Var::Real(r) => Self {
                    text: r.to_string(),
                    binding: None,
                },
                Var::Integer(i) => Self {
                    text: i.to_string(),
                    binding: None,
                },
                Var::String(text) => Self {
                    text: text.to_string(),
                    binding: None,
                },
            },
        }
    }
}

impl Widget for Label {
    fn build(&mut self, build_ctx: &mut BuildCtx, state: &State) {
        if let Some(binding) = &self.binding {
            build_ctx.bind(binding);

            if let Some(var) = state.get(binding) {
                match var {
                    Var::Real(r) => self.text = r.to_string(),
                    Var::Integer(i) => self.text = i.to_string(),
                    Var::String(s) => self.text = s.to_string(),
                }
            }
        }
    }
}
