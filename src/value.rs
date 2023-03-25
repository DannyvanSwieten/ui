use crate::ui_state::UIState;

#[derive(Clone)]
pub enum Value {
    Binding(String),
    Const(Var),
}

impl Value {
    pub fn var<'this, 'ui>(&'this self, ui_state: &'ui UIState) -> &Var
    where
        'ui: 'this,
    {
        match self {
            Value::Binding(binding) => &ui_state[binding],
            Value::Const(var) => var,
        }
    }
}

impl<T> From<T> for Value
where
    T: Into<Var>,
{
    fn from(v: T) -> Self {
        Value::Const(v.into())
    }
}

macro_rules! gen_var {
    ($($name:ident($type:ty)),*) => {
        #[derive(Clone)]
        pub enum Var {
            $(
                $name($type),
            )*
        }

        impl ToString for Var {
            fn to_string(&self) -> String {
                match self {
                    $(
                        Var::$name(v) => v.to_string(),
                    )*
                }
            }
        }

        $(
            impl From<$type> for Var {
                fn from(v: $type) -> Self {
                    Var::$name(v)
                }
            }
        )*
    };
}

gen_var!(Real(f32), Integer(i32), String(String));

impl From<&str> for Var {
    fn from(v: &str) -> Self {
        Var::String(v.to_string())
    }
}
