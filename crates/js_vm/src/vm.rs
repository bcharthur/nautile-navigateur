use crate::value::JsValue;
use crate::call_frame::CallFrame;

pub struct Vm {
    pub stack: Vec<JsValue>,
    pub frames: Vec<CallFrame>,
}

impl Vm {
    pub fn new() -> Self {
        Self { stack: Vec::new(), frames: Vec::new() }
    }

    pub fn push(&mut self, v: JsValue) { self.stack.push(v); }
    pub fn pop(&mut self) -> JsValue { self.stack.pop().unwrap_or(JsValue::Undefined) }
    pub fn peek(&self) -> &JsValue { self.stack.last().unwrap_or(&JsValue::Undefined) }
}

impl Default for Vm { fn default() -> Self { Self::new() } }
