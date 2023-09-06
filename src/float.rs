#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Float32(u32);
#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Float64(u64);

impl Float32 {
    pub fn get(&self) -> f32 {
        unsafe {
            let p = &self.0 as *const u32 as *const f32;
            *p
        }
    }

    pub fn set(&mut self, val: f32) {
        let val = unsafe {
            let p = &val as *const f32 as *const u32;
            *p
        };
        self.0 = val;
    }

    fn new(val: f32) -> Self {
        let val = unsafe {
            let p = &val as *const f32 as *const u32;
            *p
        };
        Self(val)
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
        unsafe {
            let p = &self.0 as *const u64 as *const f64;
            *p
        }
    }

    pub fn set(&mut self, val: f64) {
        let val = unsafe {
            let p = &val as *const f64 as *const u64;
            *p
        };
        self.0 = val;
    }

    fn new(val: f64) -> Self {
        let val = unsafe {
            let p = &val as *const f64 as *const u64;
            *p
        };
        Self(val)
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
