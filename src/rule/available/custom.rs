use super::super::boxed::ErasedRule;
use super::Message;
use crate::{FromValue, Rule};

#[derive(Clone)]
pub struct Custom<M>(ErasedRule<M>);

impl<V, M> Rule<V> for Custom<M>
where
    M: Into<String> + Clone + 'static,
{
    type Message = M;

    fn name(&self) -> &'static str {
        "custom"
    }

    fn call(&mut self, data: &mut crate::ValueMap) -> Result<(), Self::Message> {
        self.0.call(data)
    }
}

impl<M> Custom<M> {
    /// load closure rule
    pub fn new<F, V>(f: F) -> Custom<M>
    where
        F: for<'a> FnOnce(&'a mut V) -> Result<(), Message>,
        F: Rule<V, Message = M>,
        V: FromValue + 'static,
        M:'static
    {
        // let f =move |data| {
        //     f(data).map_err(|err| match err.kind {
        //         super::MessageKind::Fallback(str) => Message {
        //             kind: super::MessageKind::Custom(str),
        //         },
        //         _ => err,
        //     })
        // };

        Self(ErasedRule::new(f))
    }
}
