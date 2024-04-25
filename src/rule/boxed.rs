use std::marker::PhantomData;

use async_trait::async_trait;

use crate::value::ValueMap;

use super::Rule;

pub struct ErasedRule<M>(pub(super) Box<dyn BoxedRule<M>>);

impl<M> ErasedRule<M> {
    pub fn new<H, S>(handler: H) -> Self
    where
        H: Rule<S, M>,
        S: 'static,
        M: 'static,
    {
        Self(Box::new(handler.into_boxed()))
    }

    pub fn name(&self) -> &'static str {
        self.0.name()
    }
    pub async fn call(&mut self, data: &'static mut ValueMap) -> Result<(), M> {
        self.0.call(data).await
    }

    pub fn map<M2>(self, layer: fn(M) -> M2) -> ErasedRule<M2>
    where
        M: 'static,
        M2: 'static,
    {
        ErasedRule(Box::new(Map { inner: self, layer }))
    }
}

impl<M> Clone for ErasedRule<M> {
    fn clone(&self) -> Self {
        Self(self.0.clone_box())
    }
}

#[async_trait]
pub trait BoxedRule<M>: Send {
    fn clone_box(&self) -> Box<dyn BoxedRule<M>>;

    async fn call(&mut self, data: &'static mut ValueMap) -> Result<(), M>;

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

#[async_trait]
impl<H, M, T> BoxedRule<M> for RuleIntoBoxed<H, M, T>
where
    H: Rule<T, M> + Clone + Send,
    T: 'static,
    M: 'static,
{
    fn clone_box(&self) -> Box<dyn BoxedRule<M>> {
        Box::new(self.clone())
    }

    async fn call(&mut self, data: &'static mut ValueMap) -> Result<(), M> {
        self.handler.call(data).await
    }

    fn name(&self) -> &'static str {
        self.handler.name()
    }
}

pub struct Map<M, M2> {
    inner: ErasedRule<M>,
    layer: fn(M) -> M2,
}

impl<M, M2> Clone for Map<M, M2> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            layer: self.layer,
        }
    }
}

#[async_trait]
impl<M, M2> BoxedRule<M2> for Map<M, M2>
where
    M: 'static,
    M2: 'static,
{
    fn clone_box(&self) -> Box<dyn BoxedRule<M2>> {
        Box::new(self.clone())
    }

    async fn call(&mut self, data: &'static mut ValueMap) -> Result<(), M2> {
        self.inner.call(data).await.map_err(self.layer)
    }

    fn name(&self) -> &'static str {
        self.inner.name()
    }
}
