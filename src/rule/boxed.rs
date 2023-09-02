use crate::ser::ValueMap;

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
