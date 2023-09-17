use std::marker::PhantomData;

use crate::value::ValueMap;

use super::Rule;

pub struct ErasedRule<M>(pub(super) Box<dyn BoxedRule<M>>, PhantomData<M>);

impl<M> ErasedRule<M> {
    pub fn new<H, S>(handler: H) -> Self
    where
        H: Rule<S, Message = M>,
        S: 'static,
        M: 'static,
    {
        Self(Box::new(handler.into_boxed()), PhantomData)
    }

    pub fn name(&self) -> &'static str {
        self.0.name()
    }
    pub fn call(&mut self, data: &mut ValueMap) -> Result<(), M> {
        self.0.call(data)
    }
}

impl<M> Clone for ErasedRule<M> {
    fn clone(&self) -> Self {
        Self(self.0.clone_box(), PhantomData)
    }
}

pub trait BoxedRule<M> {
    fn clone_box(&self) -> Box<dyn BoxedRule<M>>;

    fn call(&mut self, data: &mut ValueMap) -> Result<(), M>;

    fn name(&self) -> &'static str;
}

pub struct RuleIntoBoxed<H, M, T> {
    handler: H,
    other: PhantomData<fn() -> T>,
    msg: PhantomData<fn() -> M>,
}

impl<H, M, T> RuleIntoBoxed<H, M, T> {
    pub(super) fn new(handler: H) -> Self {
        Self {
            handler,
            other: PhantomData,
            msg: PhantomData,
        }
    }
}

impl<H, M, T> Clone for RuleIntoBoxed<H, M, T>
where
    H: Clone,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            other: PhantomData,
            msg: PhantomData,
        }
    }
}

impl<H, M, T> BoxedRule<M> for RuleIntoBoxed<H, M, T>
where
    H: Rule<T, Message = M> + Clone,
    T: 'static,
    M: 'static,
{
    fn clone_box(&self) -> Box<dyn BoxedRule<M>> {
        Box::new(self.clone())
    }
    fn call(&mut self, data: &mut ValueMap) -> Result<(), M> {
        self.handler.call(data)
    }
    fn name(&self) -> &'static str {
        self.handler.name()
    }
}
