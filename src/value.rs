#[derive(Clone)]

pub enum Value {
    Binding(String),
    Const(Var),
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

gen_var!(
    Real(f32),
    Integer(i32),
    String(String),
    StringLiteral(&'static str)
);
