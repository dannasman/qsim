use std::fmt;
use std::ops::*;

use crate::complex::*;

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct c64x4([c64; 4]);

impl c64x4 {
    pub fn new(z11: c64, z12: c64, z21: c64, z22: c64) -> Self {
        Self([z11, z12, z21, z22])
    }

    pub fn conjugate_transpose(&self) -> Self {
        Self([
            self[0].conjugate(),
            self[2].conjugate(),
            self[1].conjugate(),
            self[3].conjugate(),
        ])
    }
}

impl fmt::Display for c64x4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t{}\n{}\t{}", self[0], self[1], self[2], self[3])
    }
}

impl Deref for c64x4 {
    type Target = [c64; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Add for c64x4 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self([
            self[0] + other[0],
            self[1] + other[1],
            self[2] + other[2],
            self[3] + other[3],
        ])
    }
}

impl AddAssign for c64x4 {
    fn add_assign(&mut self, other: Self) {
        *self = Self([
            self[0] + other[0],
            self[1] + other[1],
            self[2] + other[2],
            self[3] + other[3],
        ])
    }
}

impl Mul for c64x4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self([
            self[0] * other[0] + self[1] * other[2],
            self[0] * other[1] + self[0] * other[3],
            self[2] * other[0] + self[3] * other[1],
            self[2] * other[1] + self[3] * other[3],
        ])
    }
}

impl MulAssign for c64x4 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self([
            self[0] * other[0] + self[1] * other[2],
            self[0] * other[1] + self[0] * other[3],
            self[2] * other[0] + self[3] * other[1],
            self[2] * other[1] + self[3] * other[3],
        ])
    }
}

impl Sub for c64x4 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self([
            self[0] - other[0],
            self[1] - other[1],
            self[2] - other[2],
            self[3] - other[3],
        ])
    }
}

impl SubAssign for c64x4 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self([
            self[0] - other[0],
            self[1] - other[1],
            self[2] - other[2],
            self[3] - other[3],
        ])
    }
}
