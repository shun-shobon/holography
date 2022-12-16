use num::traits::{Float, FloatConst};

pub trait TriFunc<T: Float> {
    fn sin(&self, rad: T) -> T;
    fn cos(&self, rad: T) -> T;
}

pub struct TriFuncStd<T>(std::marker::PhantomData<T>);

impl<T: Float> TriFunc<T> for TriFuncStd<T> {
    #[inline]
    fn sin(&self, rad: T) -> T {
        rad.sin()
    }

    #[inline]
    fn cos(&self, rad: T) -> T {
        rad.cos()
    }
}

#[derive(Debug)]
pub struct TriFuncLut<T, const SIZE: usize> {
    sin: [T; SIZE],
    cos: [T; SIZE],
}

impl<T: Float + FloatConst, const SIZE: usize> TriFuncLut<T, SIZE> {
    pub fn new() -> Self {
        let mut sin = [T::zero(); SIZE];
        let mut cos = [T::zero(); SIZE];

        for i in 0..SIZE {
            let rad =
                (T::from(2).unwrap() * T::PI() / T::from(SIZE).unwrap()) * T::from(i).unwrap();
            sin[i] = rad.sin();
            cos[i] = rad.cos();
        }

        Self { sin, cos }
    }
}

impl<T: Float + FloatConst, const SIZE: usize> Default for TriFuncLut<T, SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Float + FloatConst, const SIZE: usize> TriFunc<T> for TriFuncLut<T, SIZE> {
    #[inline]
    fn sin(&self, rad: T) -> T {
        let k = (rad.fract() * T::from(SIZE).unwrap()).to_usize().unwrap();
        self.sin[k]
    }

    #[inline]
    fn cos(&self, rad: T) -> T {
        let k = (rad.fract() * T::from(SIZE).unwrap()).to_usize().unwrap();
        self.cos[k]
    }
}
