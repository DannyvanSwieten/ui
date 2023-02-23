#[derive(Clone, Copy)]
pub enum Var {
    Real(f32),
    Integer(i32),
    String(&'static str),
}

pub enum Value {
    Binding(String),
    Const(Var),
}

impl ToString for Var {
    fn to_string(&self) -> String {
        match self {
            Var::Real(r) => r.to_string(),
            Var::Integer(i) => i.to_string(),
            Var::String(s) => s.to_string(),
        }
    }
}
