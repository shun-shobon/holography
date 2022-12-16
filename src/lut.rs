use std::f64::consts::PI;

pub trait TriFunc {
    fn sin(&self, rad: f64) -> f64;
    fn cos(&self, rad: f64) -> f64;
}

pub struct TriFuncNaive;

impl Default for TriFuncNaive {
    fn default() -> Self {
        Self::new()
    }
}

impl TriFuncNaive {
    pub fn new() -> Self {
        Self
    }
}

impl TriFunc for TriFuncNaive {
    #[inline]
    fn sin(&self, rad: f64) -> f64 {
        (rad.fract() * 2.0 * PI).sin()
    }

    #[inline]
    fn cos(&self, rad: f64) -> f64 {
        (rad.fract() * 2.0 * PI).cos()
    }
}

#[derive(Debug)]
pub struct TriFuncLut<const SIZE: usize> {
    sin: [f64; SIZE],
    cos: [f64; SIZE],
}

impl<const SIZE: usize> TriFuncLut<SIZE> {
    pub fn new() -> Self {
        let mut sin = [0.0; SIZE];
        let mut cos = [0.0; SIZE];

        for i in 0..SIZE {
            let rad = (2.0 * PI / SIZE as f64) * i as f64;
            sin[i] = rad.sin();
            cos[i] = rad.cos();
        }

        Self { sin, cos }
    }
}

impl<const SIZE: usize> Default for TriFuncLut<SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const SIZE: usize> TriFunc for TriFuncLut<SIZE> {
    #[inline]
    fn sin(&self, rad: f64) -> f64 {
        let k = (rad.fract() * SIZE as f64) as usize;
        self.sin[k]
    }

    #[inline]
    fn cos(&self, rad: f64) -> f64 {
        let k = (rad.fract() * SIZE as f64) as usize;
        self.cos[k]
    }
}
