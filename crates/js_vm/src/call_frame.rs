use crate::value::JsValue;

#[derive(Debug)]
pub struct CallFrame {
    pub function_name: String,
    pub ip: usize,
    pub stack_base: usize,
    pub locals: Vec<JsValue>,
}

impl CallFrame {
    pub fn new(name: impl Into<String>, stack_base: usize) -> Self {
        Self {
            function_name: name.into(),
            ip: 0,
            stack_base,
            locals: Vec::new(),
        }
    }
}
