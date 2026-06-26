use crate::value::ObjectRef;

pub struct Realm {
    pub global_object: ObjectRef,
    pub intrinsics: Intrinsics,
}

pub struct Intrinsics {
    pub object_prototype: ObjectRef,
    pub function_prototype: ObjectRef,
    pub array_prototype: ObjectRef,
    pub string_prototype: ObjectRef,
    pub number_prototype: ObjectRef,
    pub boolean_prototype: ObjectRef,
    pub symbol_prototype: ObjectRef,
    pub error_prototype: ObjectRef,
}
