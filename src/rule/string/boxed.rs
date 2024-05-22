use std::marker::PhantomData;

use super::StringRule;

pub struct ErasedRule<M>(pub(super) Box<dyn BoxedRule<M>>);

impl<M> ErasedRule<M> {
    pub fn new<H>(handler: H) -> Self
    where
        H: StringRule<Message = M> + Clone + 'static,
        M: 'static,
    {
        Self(Box::new(handler.into_boxed()))
    }

    pub fn name(&self) -> &'static str {
        self.0.name()
    }
    pub fn call(&mut self, data: &mut String) -> Result<(), M> {
        self.0.call(data)
    }
}

pub trait BoxedRule<M> {
    fn clone_box(&self) -> Box<dyn BoxedRule<M>>;

    fn call(&mut self, data: &mut String) -> Result<(), M>;

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

impl<H: Clone, M> Clone for RuleIntoBoxed<H, M> {
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, M> BoxedRule<M> for RuleIntoBoxed<H, M>
where
    H: StringRule<Message = M> + Clone,
    M: 'static,
{
    fn clone_box(&self) -> Box<dyn BoxedRule<M>> {
        Box::new(self.clone())
    }

    fn call(&mut self, data: &mut String) -> Result<(), M> {
        self.handler.call(data)
    }

    fn name(&self) -> &'static str {
        H::NAME
    }
}
