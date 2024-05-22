use boxed::{ErasedRule, RuleIntoBoxed};

mod boxed;

pub trait StringRule: Sized + 'static + Clone {
    /// custom define returning message type
    type Message;

    /// Named rule type, used to distinguish different rules
    ///
    /// allow `a-z` | `A-Z` | `0-9` | `_` composed string, and not start with `0-9`
    const NAME: &'static str;

    /// Rule specific implementation, data is current field's value
    #[must_use]
    fn call(&mut self, data: &mut String) -> Result<(), Self::Message>;

    fn into_boxed(self) -> RuleIntoBoxed<Self, Self::Message> {
        RuleIntoBoxed::new(self)
    }
}

mod private {
    use super::StringRule;

    pub trait Sealed {}

    impl<R, M> Sealed for R where R: StringRule<Message = M> {}
}

pub trait StringRuleExt<M>: private::Sealed {
    fn and<R>(self, other: R) -> StringRuleList<M>
    where
        R: StringRule<Message = M>;

    fn custom<F>(self, other: F) -> StringRuleList<M>
    where
        F: for<'a> FnOnce(&'a mut String) -> Result<(), M> + Clone + 'static;
}

impl<S, M> StringRuleExt<M> for S
where
    S: StringRule<Message = M>,
    M: 'static,
{
    fn and<S2>(self, other: S2) -> StringRuleList<M>
    where
        S2: StringRule<Message = M>,
    {
        let is_dup = {
            if S::NAME != S2::NAME {
                false
            } else {
                !matches!(S::NAME, "custom")
            }
        };
        StringRuleList {
            list: if is_dup {
                vec![ErasedRule::new(self)]
            } else {
                vec![ErasedRule::<M>::new(self), ErasedRule::new(other)]
            },
            ..Default::default()
        }
    }

    fn custom<F>(self, other: F) -> StringRuleList<M>
    where
        F: for<'a> FnOnce(&'a mut String) -> Result<(), M> + Clone + 'static,
    {
        StringRuleList {
            list: vec![ErasedRule::new(self), ErasedRule::new(other)],
            ..Default::default()
        }
    }
}

pub struct StringRuleList<M> {
    list: Vec<ErasedRule<M>>,
    is_bail: bool,
}

impl<M> Default for StringRuleList<M> {
    fn default() -> Self {
        Self {
            list: Vec::new(),
            is_bail: false,
        }
    }
}

impl<M> StringRuleList<M> {
    pub fn remove_duplicate(&mut self, other: &ErasedRule<M>) {
        let name = other.name();

        let duplicate_rules: Vec<usize> = self
            .list
            .iter()
            .enumerate()
            .filter(|(_index, exist_rule)| {
                if exist_rule.name() != name {
                    return false;
                }
                !matches!(name, "custom")
            })
            .map(|(index, _)| index)
            .rev()
            .collect();

        for index in duplicate_rules {
            // Use `swap_remove` to get better performence because we don't
            // mind the order of rule list. If the order should be kept in
            // the future, please use `remove` instead of `swap_remove`.
            self.list.swap_remove(index);
        }
    }
    pub fn and<R>(mut self, other: R) -> Self
    where
        R: StringRule<Message = M> + Clone,
        M: 'static,
    {
        let other = ErasedRule::new(other);
        self.remove_duplicate(&other);

        self.list.push(other);
        self
    }

    pub fn custom<F>(mut self, other: F) -> Self
    where
        F: for<'a> FnOnce(&'a mut String) -> Result<(), M> + Clone + 'static,
        M: 'static,
    {
        self.list.push(ErasedRule::new(other));
        self
    }

    /// when first validate error is encountered, right away return Err(message) in one field.
    ///
    /// when [`Validator`] set bail, it will cover, and comply with [`Validator`]
    ///
    /// [`Validator`]: crate::Validator
    pub fn bail(mut self) -> Self {
        self.is_bail = true;
        self
    }

    pub(crate) fn set_bail(&mut self) {
        self.is_bail = true;
    }

    pub fn is_bail(&self) -> bool {
        self.is_bail
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    #[must_use]
    pub(crate) fn call(self, data: &mut String) -> Vec<(&'static str, M)> {
        let StringRuleList { mut list, is_bail } = self;
        let mut msg = Vec::with_capacity(list.len());

        for endpoint in list.iter_mut() {
            let _ = endpoint
                .call(data)
                .map_err(|m| msg.push((endpoint.name(), m)));

            if is_bail && !msg.is_empty() {
                msg.shrink_to(1);
                return msg;
            }
        }

        msg.shrink_to_fit();
        msg
    }
}

trait IntoStringRuleList<M> {
    fn into_list(self) -> StringRuleList<M>;
}

fn custom<F, M>(f: F) -> StringRuleList<M>
where
    F: FnOnce(&mut String) -> Result<(), M> + Clone + 'static,
    M: 'static,
{
    StringRuleList {
        list: vec![ErasedRule::new(f)],
        ..Default::default()
    }
}

impl<M> IntoStringRuleList<M> for StringRuleList<M> {
    fn into_list(self) -> StringRuleList<M> {
        self
    }
}

impl<R, M> IntoStringRuleList<M> for R
where
    R: StringRule<Message = M>,
    M: 'static,
{
    fn into_list(self) -> StringRuleList<M> {
        StringRuleList {
            list: vec![ErasedRule::new(self)],
            ..Default::default()
        }
    }
}

impl<F, M> StringRule for F
where
    F: FnOnce(&mut String) -> Result<(), M> + Clone + 'static,
{
    type Message = M;

    const NAME: &'static str = "custom";

    fn call(&mut self, data: &mut String) -> Result<(), M> {
        self.clone()(data)
    }
}

#[cfg(all(test, feature = "full"))]
mod test_regster {

    use super::super::available::*;
    use super::*;
    fn register<R: IntoStringRuleList<M>, M>(_: R) {}
    fn register2<R: IntoStringRuleList<Message>>(_: R) {}

    fn hander(_val: &mut String) -> Result<(), Message> {
        Ok(())
    }

    #[derive(Clone)]
    struct Gt10;

    impl StringRule for Gt10 {
        type Message = u8;

        const NAME: &'static str = "gt10";
        fn call(&mut self, data: &mut String) -> Result<(), u8> {
            todo!()
        }
    }

    #[test]
    fn test() {
        assert_eq!(Gt10::NAME, "gt10");

        register(Required);
        register(Required.custom(hander));
        register(Required.and(StartWith("foo")));
        register(Required.and(StartWith("foo")).bail());
        register(Required.and(StartWith("foo")).custom(hander).bail());
        register(Required.and(StartWith("foo")).custom(hander).bail());

        register(custom(hander));
        register2(custom(hander));

        register(custom(hander).and(StartWith("foo")));
        register(custom(hander).and(StartWith("foo")).bail());
        // register(custom(|_a: &mut u8| Ok(())).and(Gt10));
        // register(Gt10.custom(|_a: &mut u8| Ok(())));
    }
}
