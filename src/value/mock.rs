use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub enum Value<'v> {
    Uint8(u8),
    Int8(i8),
    Uint16(u16),
    Int16(i16),
    Uint32(u32),
    Int32(i32),
    Uint64(u64),
    Int64(i64),
    // Float32(float::Float32),
    // Float64(float::Float64),
    String(String),
    Str(&'v str),
    Unit,
    Boolean(bool),
    Char(char),
    Bytes(Vec<u8>),

    // fn unimplemented
    // i128 u128 unimplemented
    // ISize(isize), unimplemented
    // USize(usize), unimplemented
    // pointer, Raw pointer unimplemented
    #[doc(hidden)]
    Option(Box<Option<Value<'v>>>),

    #[doc(hidden)]
    Array(Vec<Value<'v>>),

    #[doc(hidden)]
    Tuple(Vec<Value<'v>>),

    #[doc(hidden)]
    TupleStruct(Vec<Value<'v>>),

    #[doc(hidden)]
    NewtypeStruct(Vec<Value<'v>>),

    #[doc(hidden)]
    Enum(&'static str, Vec<Value<'v>>),
    #[doc(hidden)]
    EnumUnit(&'static str),
    #[doc(hidden)]
    TupleVariant(&'static str, Vec<Value<'v>>),

    #[doc(hidden)]
    Map(BTreeMap<Value<'v>, Value<'v>>),

    #[doc(hidden)]
    StructKey(String),
    /// the BtreeMap key only be StructKey(_)
    #[doc(hidden)]
    Struct(BTreeMap<Value<'v>, Value<'v>>),

    #[doc(hidden)]
    StructVariantKey(String),
    /// the BtreeMap key only be StructVariantKey(_)
    #[doc(hidden)]
    StructVariant(&'static str, BTreeMap<Value<'v>, Value<'v>>),
}
