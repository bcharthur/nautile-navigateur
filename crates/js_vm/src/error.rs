use crate::value::JsValue;
use std::fmt;

#[derive(Debug, Clone)]
pub struct JsError {
    pub kind: JsErrorKind,
    pub message: String,
    pub value: Option<JsValue>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JsErrorKind {
    TypeError,
    ReferenceError,
    SyntaxError,
    RangeError,
    EvalError,
    URIError,
    InternalError,
    Thrown,
}

impl fmt::Display for JsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl std::error::Error for JsError {}

pub type JsResult<T> = Result<T, JsError>;
