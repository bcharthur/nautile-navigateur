//! JavaScript runtime values, realms, and execution context skeleton.
/// Identifier for a GC-managed JS string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JsStringId(pub u32);
/// Identifier for a JS symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JsSymbolId(pub u32);
/// Identifier for a BigInt allocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JsBigIntId(pub u32);
/// Identifier for a JS object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JsObjectId(pub u32);
/// JavaScript value representation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JsValue {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    String(JsStringId),
    Symbol(JsSymbolId),
    BigInt(JsBigIntId),
    Object(JsObjectId),
}
/// JS realm with intrinsics and global object.
#[derive(Debug, Default)]
pub struct Realm {
    pub id: u64,
}
/// Active execution context.
#[derive(Debug, Default)]
pub struct ExecutionContext {
    pub realm_id: u64,
}
/// Global object marker.
#[derive(Debug, Default)]
pub struct GlobalObject;
