use std::marker::PhantomData;

use crate::value::ValueMap;

use super::Rule;

pub(super) struct BoxedIntoRule(pub Box<dyn ErasedIntoRule + 'static>);

impl BoxedIntoRule {
    pub(super) fn new<H, M>(handler: H) -> Self
    where
        H: Rule<M>,
        M: 'static,
    {
        Self(Box::new(MakeErasedHandler {
            handler,
            into_route: |h| BaseRule::new(h),
            _marker: PhantomData::<M>,
        }))
    }

    // pub(super) fn name(&self) -> &'static str {
    //     self.0.clone_box().into_rule().name()
    // }
}

impl Clone for BoxedIntoRule {
    fn clone(&self) -> Self {
        Self(self.0.clone_box())
    }
}

pub trait ErasedIntoRule {
    fn clone_box(&self) -> Box<dyn ErasedIntoRule>;
    fn into_rule(self: Box<Self>) -> BaseRule;
}

pub(super) struct MakeErasedHandler<H, M> {
    pub(super) handler: H,
    pub(super) into_route: fn(H) -> BaseRule,
    _marker: PhantomData<M>,
}

impl<H, M> ErasedIntoRule for MakeErasedHandler<H, M>
where
    H: Rule<M> + Clone,
    M: 'static,
{
    fn clone_box(&self) -> Box<dyn ErasedIntoRule> {
        Box::new(self.clone())
    }
    fn into_rule(self: Box<Self>) -> BaseRule {
        (self.into_route)(self.handler)
    }
}

impl<H: Clone, M> Clone for MakeErasedHandler<H, M> {
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            into_route: self.into_route,
            _marker: PhantomData,
        }
    }
}

pub struct BaseRule(pub(super) Box<dyn RuleService>);

impl BaseRule {
    pub fn new<H, M>(handler: H) -> Self
    where
        H: Rule<M>,
        M: 'static,
    {
        Self(Box::new(handler.into_serve()))
    }
}

impl Clone for BaseRule {
    fn clone(&self) -> Self {
        Self(self.0.clone_box())
    }
}

pub trait RuleService {
    fn clone_box(&self) -> Box<dyn RuleService>;

    fn call(&mut self, data: &mut ValueMap) -> Result<(), String>;

    fn name(&self) -> &'static str;
}

pub struct RuleIntoService<H, M> {
    handler: H,
    _marker: PhantomData<fn() -> M>,
}

impl<H, M> RuleIntoService<H, M> {
    pub(super) fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, M> Clone for RuleIntoService<H, M>
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

impl<H, M> RuleService for RuleIntoService<H, M>
where
    H: Rule<M> + Clone,
    M: 'static,
{
    fn clone_box(&self) -> Box<dyn RuleService> {
        Box::new(self.clone())
    }
    fn call(&mut self, data: &mut ValueMap) -> Result<(), String> {
        self.handler.call(data).map_err(|e| e.into())
    }
    fn name(&self) -> &'static str {
        self.handler.name()
    }
}
