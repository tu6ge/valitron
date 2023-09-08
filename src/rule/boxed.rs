use std::marker::PhantomData;

use crate::value::ValueMap;

use super::Rule;

pub struct ErasedRule(pub(super) Box<dyn BoxedRule>);

impl ErasedRule {
    pub fn new<H, M>(handler: H) -> Self
    where
        H: Rule<M>,
        M: 'static,
    {
        Self(Box::new(handler.into_serve()))
    }
}

impl Clone for ErasedRule {
    fn clone(&self) -> Self {
        Self(self.0.clone_box())
    }
}

pub trait BoxedRule {
    fn clone_box(&self) -> Box<dyn BoxedRule>;

    fn call(&mut self, data: &mut ValueMap) -> Result<(), String>;

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
    fn call(&mut self, data: &mut ValueMap) -> Result<(), String> {
        self.handler.call(data).map_err(|e| e.into())
    }
    fn name(&self) -> &'static str {
        self.handler.name()
    }
}
