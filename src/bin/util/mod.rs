use std::ops::Sub;

use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct SimpleVector {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl SimpleVector {
    pub(crate) fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Add for SimpleVector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y))
    }
}

impl Add for &SimpleVector {
    type Output = SimpleVector;

    fn add(self, rhs: Self) -> Self::Output {
        SimpleVector::new(self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y))
    }
}

impl Sub for SimpleVector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x.wrapping_sub(rhs.x), self.y.wrapping_sub(rhs.y))
    }
}

impl Sub for &SimpleVector {
    type Output = SimpleVector;

    fn sub(self, rhs: Self) -> Self::Output {
        SimpleVector::new(self.x.wrapping_sub(rhs.x), self.y.wrapping_sub(rhs.y))
    }
}
