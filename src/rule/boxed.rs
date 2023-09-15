use std::marker::PhantomData;

use crate::value::ValueMap;

use super::{IntoRuleMessage, Message, Rule};

pub struct ErasedRule<M>(pub(super) Box<dyn BoxedRule>, PhantomData<M>);

impl<M> ErasedRule<M> {
    pub fn new<H, S>(handler: H) -> Self
    where
        H: Rule<S, Message = M>,
        S: 'static,
        M: IntoRuleMessage + 'static,
    {
        Self(Box::new(handler.into_boxed()), PhantomData)
    }

    pub fn name(&self) -> &'static str {
        self.0.name()
    }
    pub fn call(&mut self, data: &mut ValueMap) -> Result<(), Message> {
        self.0.call(data)
    }
}

impl<M> Clone for ErasedRule<M> {
    fn clone(&self) -> Self {
        Self(self.0.clone_box(), PhantomData)
    }
}

pub trait BoxedRule {
    fn clone_box(&self) -> Box<dyn BoxedRule>;

    fn call(&mut self, data: &mut ValueMap) -> Result<(), Message>;

    fn name(&self) -> &'static str;
}

pub struct RuleIntoBoxed<H, M> {
    handler: H,
    _marker: PhantomData<fn() -> M>,
}

impl<H, M> RuleIntoBoxed<H, M> {
    pub(super) fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, M> Clone for RuleIntoBoxed<H, M>
where
    H: Clone,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, M> BoxedRule for RuleIntoBoxed<H, M>
where
    H: Rule<M> + Clone,
    M: 'static,
{
    fn clone_box(&self) -> Box<dyn BoxedRule> {
        Box::new(self.clone())
    }
    fn call(&mut self, data: &mut ValueMap) -> Result<(), Message> {
        self.handler
            .call(data)
            .map_err(IntoRuleMessage::into_message)
    }
    fn name(&self) -> &'static str {
        self.handler.name()
    }
}
