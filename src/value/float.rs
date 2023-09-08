#[derive(Default, Clone)]
/// Wrapper of `f32`, marked as implementation of `Eq` and `Ord`, but avoid user to use them.
/// Just alow in "Value", but not "Key"
pub struct Float32(pub(super) f32);
#[derive(Default, Clone)]
/// Wrapper of `f64`, marked as implementation of `Eq` and `Ord`, but avoid user to use them.
/// Just alow in "Value", but not "Key"
pub struct Float64(pub(super) f64);

impl Float32 {
    pub fn get(&self) -> f32 {
        self.0
    }

    pub fn set(&mut self, val: f32) {
        self.0 = val;
    }

    pub fn new(val: f32) -> Self {
        Self(val)
    }
}

impl PartialEq for Float32 {
    fn eq(&self, other: &Self) -> bool {
        self.get().eq(&other.get())
    }
}

impl PartialOrd for Float32 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get().partial_cmp(&other.get())
    }
}

impl Eq for Float32 {}

impl Ord for Float32 {
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
        panic!("never invoke this")
    }
}

impl From<f32> for Float32 {
    fn from(value: f32) -> Self {
        Float32::new(value)
    }
}

impl From<Float32> for f32 {
    fn from(value: Float32) -> Self {
        value.get()
    }
}

impl std::fmt::Debug for Float32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

impl Float64 {
    pub fn get(&self) -> f64 {
        self.0
    }

    pub fn set(&mut self, val: f64) {
        self.0 = val;
    }

    pub fn new(val: f64) -> Self {
        Self(val)
    }
}

impl PartialEq for Float64 {
    fn eq(&self, other: &Self) -> bool {
        self.get().eq(&other.get())
    }
}

impl PartialOrd for Float64 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get().partial_cmp(&other.get())
    }
}

impl Eq for Float64 {}

impl Ord for Float64 {
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
        panic!("never invoke this")
    }
}

impl From<f64> for Float64 {
    fn from(value: f64) -> Self {
        Float64::new(value)
    }
}

impl From<Float64> for f64 {
    fn from(value: Float64) -> Self {
        value.get()
    }
}

impl std::fmt::Debug for Float64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn painc_on_ord_float32() {
        let mut h = std::collections::BTreeMap::new();
        h.insert(Float32::new(10.0), 10);
        h.insert(Float32::new(20.0), 20);
    }

    #[test]
    #[should_panic]
    fn painc_on_ord_float64() {
        let mut h = std::collections::BTreeMap::new();
        h.insert(Float64::new(10.0), 10);
        h.insert(Float64::new(20.0), 20);
    }

    #[test]
    fn test_eq_f32() {
        let a = Float32::new(10.0);
        let b = Float32::new(10.0);
        assert_eq!(a, b);

        let c = Float32(f32::NAN);
        let d = Float32(f32::NAN);
        assert!(d != c);

        let e = Float32(0.0);
        let f = Float32(-0.0);
        assert!(f == e);
    }

    #[test]
    fn test_ord_f32() {
        let a = Float32::new(10.0);
        let b = Float32::new(20.0);
        assert!(b > a);
    }

    #[test]
    fn test_eq_f64() {
        let a = Float64::new(10.0);
        let b = Float64::new(10.0);
        assert_eq!(a, b);

        let c = Float64(f64::NAN);
        let d = Float64(f64::NAN);
        assert!(d != c);

        let e = Float64(0.0);
        let f = Float64(-0.0);
        assert!(f == e);
    }

    #[test]
    fn test_ord_f64() {
        let a = Float64::new(10.0);
        let b = Float64::new(20.0);
        assert!(b > a);
    }
}
