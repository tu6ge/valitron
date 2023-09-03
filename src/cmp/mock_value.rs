#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub enum Value {
    Int8(i8),
    Uint8(u8),
    String(String),
    Str(&'static str),
    Unit,
}
