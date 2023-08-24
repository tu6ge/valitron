use std::{
    collections::HashMap, convert::Infallible, fmt::Display, future::Future, ops::BitOr, pin::Pin,
};

use serde::{de::IntoDeserializer, ser, Deserialize};
use serde_json::Value;

trait Rule: 'static {}

#[derive(Debug)]
struct Request;
#[derive(Debug)]
struct StartWith;
#[derive(Debug)]
struct Trim;

impl Rule for Request {}
impl Rule for StartWith {}
impl Rule for Trim {}

#[derive(Default)]
struct RuleList(Vec<Box<dyn Rule>>);

impl RuleList {
    pub fn bail(mut self) -> Self {
        self
    }
}

pub fn main() {}

fn register<F>(name: &str, f: F)
where
    F: RegisterRule,
{
}

trait RegisterRule {}

impl<F> Rule for F where F: FnOnce(Value) -> bool + 'static {}

impl Rule for &'static str {}

struct Data<T> {
    inner: T,
}

impl<T> tower::Service<Data<T>> for Request {
    type Response = bool;

    type Error = String;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Data<T>) -> Self::Future {
        Box::pin(async { Ok(true) })
    }
}

trait RuleBox {}

impl<T: Rule, U: Rule> RuleBox for (T, U) {}
impl<T: Rule, U: Rule, V: Rule> RuleBox for (T, U, V) {}

trait RuleExt {
    fn and<R: Rule>(self, other: R) -> RuleList;
}

impl<T: Rule> RuleExt for T {
    fn and<R: Rule>(self, other: R) -> RuleList {
        RuleList(vec![Box::new(self), Box::new(other)])
    }
}

impl RuleExt for RuleList {
    fn and<R: Rule>(self, other: R) -> RuleList {
        let RuleList(mut vec) = self;
        vec.push(Box::new(other));

        RuleList(vec)
    }
}

impl<T: Rule> From<T> for RuleList {
    fn from(value: T) -> Self {
        Self(vec![Box::new(value)])
    }
}

mod other {
    use crate::{Request, RuleExt, RuleList, StartWith, Trim};

    fn register<T: Into<RuleList>>(list: T) {}

    //#[test]
    fn tesst() {
        let list = Request.and(Trim).and(StartWith);
        println!("{}", list.0.len());

        register(Request.and(Trim).and(StartWith).and(|val| true));
        register(Request.and(Trim).and(StartWith).bail().and(|val| true));
        register(Request);
        register(|_| false)
    }
}
