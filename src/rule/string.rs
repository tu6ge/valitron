use super::{CoreRule, RuleList};

pub fn custom<F, M>(f: F) -> RuleList<String, M>
where
    F: FnOnce(&mut String) -> Result<(), M> + Clone + 'static,
    M: 'static,
{
    RuleList::from_fn(f)
}

mod private {
    use crate::rule::CoreRule;

    pub trait Sealed {}

    impl<R, M> Sealed for R where R: CoreRule<String, (), Message = M> {}
}

pub trait StringRuleExt<M>: private::Sealed {
    fn and<R>(self, other: R) -> RuleList<String, M>
    where
        R: CoreRule<String, (), Message = M>;

    fn custom<F>(self, other: F) -> RuleList<String, M>
    where
        F: FnOnce(&mut String) -> Result<(), M> + Clone + 'static;
}

impl<S, M> StringRuleExt<M> for S
where
    S: CoreRule<String, (), Message = M>,
    M: 'static,
{
    fn and<S2>(self, other: S2) -> RuleList<String, M>
    where
        S2: CoreRule<String, (), Message = M>,
    {
        RuleList::from_ext_and(self, other)
    }

    fn custom<F>(self, fun: F) -> RuleList<String, M>
    where
        F: FnOnce(&mut String) -> Result<(), M> + Clone + 'static,
    {
        RuleList::append_fn(self, fun)
    }
}
