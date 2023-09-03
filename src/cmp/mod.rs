mod mock_value;

use std::{cmp::Ordering, ops::RangeBounds, string::Drain};

use mock_value::Value;

impl PartialEq<i8> for Value {
    fn eq(&self, other: &i8) -> bool {
        if let Value::Int8(n) = self {
            n.eq(other)
        } else {
            false
        }
    }
}

impl PartialOrd<i8> for Value {
    fn partial_cmp(&self, other: &i8) -> Option<Ordering> {
        if let Value::Int8(n) = self {
            n.partial_cmp(other)
        } else {
            None
        }
    }
}

macro_rules! tyerr {
    () => {
        panic!("type mismatch")
    };
}

impl Value {
    pub fn push_str(&mut self, string: &str) {
        if let Value::String(s) = self {
            s.push_str(string)
        } else {
            tyerr!()
        }
    }
    pub fn push(&mut self, ch: char) {
        if let Value::String(s) = self {
            s.push(ch)
        } else {
            tyerr!()
        }
    }
    pub fn truncate(&mut self, new_len: usize) {
        if let Value::String(s) = self {
            s.truncate(new_len)
        } else {
            tyerr!()
        }
    }
    pub fn pop(&mut self) -> Option<char> {
        if let Value::String(s) = self {
            s.pop()
        } else {
            tyerr!()
        }
    }
    pub fn remove(&mut self, idx: usize) -> char {
        if let Value::String(s) = self {
            s.remove(idx)
        } else {
            tyerr!()
        }
    }
    pub fn insert(&mut self, idx: usize, ch: char) {
        if let Value::String(s) = self {
            s.insert(idx, ch)
        } else {
            tyerr!()
        }
    }
    pub fn insert_str(&mut self, idx: usize, string: &str) {
        if let Value::String(s) = self {
            s.insert_str(idx, string)
        } else {
            tyerr!()
        }
    }

    /// TODO other type
    pub fn len(&self) -> usize {
        if let Value::String(s) = self {
            s.len()
        } else {
            todo!()
        }
    }
    pub fn is_empty(&self) -> bool {
        if let Value::String(s) = self {
            s.is_empty()
        } else {
            false
        }
    }
    pub fn drain<R>(&mut self, range: R) -> Drain<'_>
    where
        R: RangeBounds<usize>,
    {
        if let Value::String(s) = self {
            s.drain(range)
        } else {
            tyerr!()
        }
    }
    pub fn replace_range<R>(&mut self, range: R, replace_with: &str)
    where
        R: RangeBounds<usize>,
    {
        if let Value::String(s) = self {
            s.replace_range(range, replace_with)
        } else {
            tyerr!()
        }
    }

    //----------------------------- u8 -----------------------------------
    pub const fn is_ascii(&self) -> bool {
        if let Value::Uint8(s) = self {
            s.is_ascii()
        } else {
            false
        }
    }
    pub const fn to_ascii_uppercase(&self) -> u8 {
        if let Value::Uint8(s) = self {
            s.to_ascii_uppercase()
        } else {
            tyerr!()
        }
    }
    pub const fn to_ascii_lowercase(&self) -> u8 {
        if let Value::Uint8(s) = self {
            s.to_ascii_lowercase()
        } else {
            tyerr!()
        }
    }
    pub fn make_ascii_uppercase(&mut self) {
        if let Value::Uint8(s) = self {
            s.make_ascii_uppercase()
        } else {
            tyerr!()
        }
    }
    pub fn make_ascii_lowercase(&mut self) {
        if let Value::Uint8(s) = self {
            s.make_ascii_lowercase()
        } else {
            tyerr!()
        }
    }
}
