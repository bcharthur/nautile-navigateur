
#[derive(Debug, Clone)]
pub enum JsValue {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    BigInt(i128),
    String(String),
    Symbol(u64),
    Object(ObjectRef),
    Function(FunctionRef),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectRef(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionRef(pub u32);

impl JsValue {
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Undefined | Self::Null => false,
            Self::Boolean(b) => *b,
            Self::Number(n) => *n != 0.0 && !n.is_nan(),
            Self::String(s) => !s.is_empty(),
            Self::BigInt(n) => *n != 0,
            Self::Symbol(_) | Self::Object(_) | Self::Function(_) => true,
        }
    }

    pub fn type_of(&self) -> &'static str {
        match self {
            Self::Undefined => "undefined",
            Self::Null => "object",
            Self::Boolean(_) => "boolean",
            Self::Number(_) => "number",
            Self::BigInt(_) => "bigint",
            Self::String(_) => "string",
            Self::Symbol(_) => "symbol",
            Self::Object(_) => "object",
            Self::Function(_) => "function",
        }
    }

    pub fn to_number(&self) -> f64 {
        match self {
            Self::Number(n) => *n,
            Self::Boolean(b) => if *b { 1.0 } else { 0.0 },
            Self::Null => 0.0,
            Self::Undefined => f64::NAN,
            Self::String(s) => s.trim().parse().unwrap_or(f64::NAN),
            _ => f64::NAN,
        }
    }

    pub fn to_string_repr(&self) -> String {
        match self {
            Self::Undefined => "undefined".into(),
            Self::Null => "null".into(),
            Self::Boolean(b) => b.to_string(),
            Self::Number(n) => format_js_number(*n),
            Self::BigInt(n) => n.to_string(),
            Self::String(s) => s.clone(),
            Self::Symbol(_) => "Symbol()".into(),
            Self::Object(_) => "[object Object]".into(),
            Self::Function(_) => "function () {}".into(),
        }
    }
}

fn format_js_number(n: f64) -> String {
    if n.is_nan() { return "NaN".into(); }
    if n.is_infinite() { return if n > 0.0 { "Infinity".into() } else { "-Infinity".into() }; }
    if n == 0.0 { return "0".into(); }
    format!("{}", n)
}
