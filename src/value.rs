#[derive(Clone)]
pub enum Var {
    Real(f32),
    Integer(i32),
    StringLiteral(&'static str),
    String(String),
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
            Var::String(s) => s.clone(),
            Var::StringLiteral(s) => s.to_string(),
        }
    }
}
