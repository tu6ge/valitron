pub trait Rule<T>: 'static + Sized + Clone {
    /// custom define returning message type
    ///
    /// u8 or String or both
    type Message;

    /// Named rule type, used to distinguish between different rules.
    ///
    /// allow `a-z` | `A-Z` | `0-9` | `_` composed string, and not start with `0-9`
    fn name(&self) -> &'static str;

    /// Rule specific implementation, data is gived type all field's value, and current field index.
    ///
    /// success returning Ok(()), or else returning message.
    fn call(&mut self, data: &mut ValueMap<'_>) -> Result<(), Self::Message>;
}

pub enum ValueMap<'a> {
    str(&'a str),
    U8(u8),
}
