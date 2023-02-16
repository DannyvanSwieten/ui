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
