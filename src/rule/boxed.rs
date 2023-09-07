use std::marker::PhantomData;

use crate::value::ValueMap;

use super::{Message, Rule};

pub(super) trait CloneRule<T>: Rule<T> {
    fn clone_box(&self) -> Box<dyn CloneRule<T>>;
}

impl<T, R> CloneRule<T> for R
where
    R: Rule<T> + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn CloneRule<T>> {
        Box::new(self.clone())
    }
}

pub(super) struct BoxCloneRule<T>(Box<dyn CloneRule<T>>);

impl<T> BoxCloneRule<T> {
    pub(super) fn new<R>(rule: R) -> Self
    where
        R: Rule<T> + Clone + 'static,
    {
        BoxCloneRule(Box::new(rule))
    }
}
impl<T: 'static> BoxCloneRule<T> {
    pub(super) fn call(&mut self, map: &mut ValueMap) -> Result<(), Message<T>> {
        self.0.call(map)
    }

    pub(super) fn name(&self) -> &'static str {
        self.0.name()
    }
}

impl<T: 'static> Clone for BoxCloneRule<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone_box())
    }
}

pub(super) struct BoxedIntoRule(Box<dyn ErasedIntoRule + 'static>);

impl BoxedIntoRule {
    pub(super) fn new<H: Rule<M> + Clone + 'static, M: 'static>(handler: H) -> Self {
        Self(Box::new(MakeErasedHandler {
            handler,
            into_route: |h| BaseRule { name: h.name() },
            _marker: PhantomData::<M>,
            //into_route: |h| BaseRule,
        }))
    }

    pub(super) fn name(&self) -> &'static str {
        self.0.clone_box().into_rule().name()
    }
}

impl Clone for BoxedIntoRule {
    fn clone(&self) -> Self {
        Self(self.0.clone_box())
    }
}

trait ErasedIntoRule {
    fn clone_box(&self) -> Box<dyn ErasedIntoRule>;
    fn into_rule(self: Box<Self>) -> BaseRule;
}

pub(super) struct MakeErasedHandler<H, M> {
    pub(super) handler: H,
    pub(super) into_route: fn(H) -> BaseRule,
    _marker: PhantomData<M>,
}

impl<H: Rule<M> + Clone + 'static, M: 'static> ErasedIntoRule for MakeErasedHandler<H, M> {
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

impl<H: Rule<M> + Clone + 'static, M> MakeErasedHandler<H, M> {
    pub(super) fn call(&mut self, map: &mut ValueMap) -> Result<(), Message<M>> {
        self.handler.call(map)
    }

    pub(super) fn name(&self) -> &'static str {
        self.handler.name()
    }
}

pub(super) struct BaseRule {
    name: &'static str,
}

impl BaseRule {
    fn name(&self) -> &'static str {
        self.name
    }
}
