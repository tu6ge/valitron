use std::marker::PhantomData;

use super::CoreRule;

pub struct ErasedRule<I, M>(pub(super) Box<dyn BoxedRule<I, M>>);

impl<I, M> ErasedRule<I, M> {
    pub fn new<H, S>(handler: H) -> Self
    where
        H: CoreRule<I, S, Message = M>,
        S: 'static,
        M: 'static,
    {
        Self(Box::new(handler.into_boxed()))
    }

    pub fn name(&self) -> &'static str {
        self.0.name()
    }
    pub fn call(&mut self, data: &mut I) -> Result<(), M> {
        self.0.call(data)
    }

    pub fn map<M2>(self, layer: fn(M) -> M2) -> ErasedRule<I, M2>
    where
        M: 'static,
        M2: 'static,
        I: 'static,
    {
        ErasedRule(Box::new(Map { inner: self, layer }))
    }
}

impl<I, M> Clone for ErasedRule<I, M> {
    fn clone(&self) -> Self {
        Self(self.0.clone_box())
    }
}

pub trait BoxedRule<I, M> {
    fn clone_box(&self) -> Box<dyn BoxedRule<I, M>>;

    fn call(&mut self, data: &mut I) -> Result<(), M>;

    fn name(&self) -> &'static str;
}

pub struct RuleIntoBoxed<H, M, T> {
    handler: H,
    _marker: PhantomData<fn() -> T>,
    _message: PhantomData<fn() -> M>,
}

impl<H, M, T> RuleIntoBoxed<H, M, T> {
    pub(super) fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
            _message: PhantomData,
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
            _marker: PhantomData,
            _message: PhantomData,
        }
    }
}

impl<H, I, M, T> BoxedRule<I, M> for RuleIntoBoxed<H, M, T>
where
    H: CoreRule<I, T, Message = M> + Clone,
    T: 'static,
    M: 'static,
{
    fn clone_box(&self) -> Box<dyn BoxedRule<I, M>> {
        Box::new(self.clone())
    }

    fn call(&mut self, data: &mut I) -> Result<(), M> {
        self.handler.call(data)
    }

    fn name(&self) -> &'static str {
        H::THE_NAME
    }
}

pub struct Map<I, M, M2> {
    inner: ErasedRule<I, M>,
    layer: fn(M) -> M2,
}

impl<I, M, M2> Clone for Map<I, M, M2> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            layer: self.layer,
        }
    }
}

impl<I, M, M2> BoxedRule<I, M2> for Map<I, M, M2>
where
    M: 'static,
    M2: 'static,
    I: 'static,
{
    fn clone_box(&self) -> Box<dyn BoxedRule<I, M2>> {
        Box::new(self.clone())
    }

    fn call(&mut self, data: &mut I) -> Result<(), M2> {
        self.inner.call(data).map_err(self.layer)
    }

    fn name(&self) -> &'static str {
        self.inner.name()
    }
}
