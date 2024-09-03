use std::fmt::Display;

type Name = String;
#[derive(Clone, Debug)]
pub enum Expr {
    String (String),
    Define {
        name: Name,
        options: Box<Expr>,
    },
    Generate(Box<Expr>),
    Display(Box<Expr>),
    Ref(Name),
    List(Vec<Expr>),
    Weighted {
        value: Box<Expr>,
        weight: u32
    },
    Join {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Null
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Define { name, .. } => format!("@{name}"),
            Self::Display(..) => "<null>".into(),
            Self::Generate(name) => format!("<generate {}>", name.to_string()),
            Self::Join { .. } => "<null>".into(),
            Self::Null => "<null>".into(),
            Self::List(values) => format!("[{}]", values.iter().map(|f| f.to_string()).collect::<Vec<String>>().join(", ")),
            Self::Ref(name) => format!("@{name}"),
            Self::String(s) => s.clone(),
            Self::Weighted { value, weight } => format!("{}^{}", value.to_string(), weight)
        }; 
        f.write_str(s.as_str())
    }
}