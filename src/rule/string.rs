use super::{CoreRule, RuleList};

pub fn custom<F, M>(f: F) -> RuleList<String, M>
where
    F: FnOnce(&mut String) -> Result<(), M> + Clone + 'static,
    M: Send + 'static,
{
    RuleList::from_fn(f)
}

mod private {
    use crate::rule::CoreRule;

    pub trait Sealed {}

    impl<R, M> Sealed for R where R: CoreRule<String, (), Message = M> {}
}

pub trait StringRuleExt<M>: private::Sealed {
    fn and<R>(self, other: R) -> RuleList<String, M>
    where
        R: CoreRule<String, (), Message = M>;

    fn custom<F>(self, other: F) -> RuleList<String, M>
    where
        F: FnOnce(&mut String) -> Result<(), M> + Clone + 'static;
}

impl<S, M> StringRuleExt<M> for S
where
    S: CoreRule<String, (), Message = M>,
    M: Send + 'static,
{
    fn and<S2>(self, other: S2) -> RuleList<String, M>
    where
        S2: CoreRule<String, (), Message = M>,
    {
        RuleList::from_ext_and(self, other)
    }

    fn custom<F>(self, fun: F) -> RuleList<String, M>
    where
        F: FnOnce(&mut String) -> Result<(), M> + Clone + 'static,
    {
        RuleList::append_fn(self, fun)
    }
}

pub trait StringRule: Clone {
    /// custom define returning message type
    type Message: Send;

    /// Named rule type, used to distinguish different rules
    ///
    /// allow `a-z` | `A-Z` | `0-9` | `_` composed string, and not start with `0-9`
    const NAME: &'static str;

    /// Default rule error message, when validate fails, return the message to user
    fn message(&self) -> Self::Message;

    /// Rule specific implementation, data is current field's value
    #[must_use]
    fn call(&mut self, data: &mut String) -> bool;
}

impl<T> CoreRule<String, ()> for T
where
    T: StringRule + 'static + Clone,
    T::Message: Send,
{
    type Message = T::Message;

    const THE_NAME: &'static str = T::NAME;

    /// Rule specific implementation, data is gived type all field's value, and current field index.
    fn call(
        &mut self,
        data: &mut String,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), Self::Message>> + Send>>
    {
        let result = if self.call(data) {
            Ok(())
        } else {
            Err(self.message())
        };
        Box::pin(async move { result })
    }
}

impl<F, M> CoreRule<String, ((), ())> for F
where
    F: for<'a> FnOnce(&'a mut String) -> Result<(), M> + 'static + Clone,
    M: Send + 'static,
{
    type Message = M;
    const THE_NAME: &'static str = "custom";

    fn call(
        &mut self,
        data: &mut String,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), Self::Message>> + Send>>
    {
        let result = self.clone()(data);
        Box::pin(async move { result })
    }
}
