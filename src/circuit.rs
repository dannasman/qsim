use std::fmt;
use std::ops::*;

use crate::complex::*;
use crate::matrix::*;

#[derive(Copy, Clone)]
struct Pointer(*mut c64);
unsafe impl Send for Pointer {}
unsafe impl Sync for Pointer {}

#[cfg(not(feature = "no-multi-thread"))]
extern crate rayon;
#[cfg(not(feature = "no-multi-thread"))]
use rayon::prelude::*;

pub const PI: f64 = std::f64::consts::PI;

pub struct QRegister(Vec<c64>);

impl QRegister {
    pub fn new(states: Vec<c64>) -> Self {
        Self(states)
    }

    pub fn apply_gate_par(&mut self, t: usize, gate: &QGate) {
        let n: usize = self.len() >> 1;

        let reg_ptr = Pointer(self.as_mut_ptr());
        (0..n).into_par_iter().for_each(|i| {
            let m: usize = (1 << t) - 1;

            let zero_state: usize = (i & m) | ((i & !m) << 1);
            let one_state: usize = zero_state | (1 << t);

            let zero_amp: c64 = self[zero_state];
            let one_amp: c64 = self[one_state];

            let zerop = unsafe { &mut *{reg_ptr}.0.add(zero_state) };
            let onep = unsafe { &mut *{reg_ptr}.0.add(one_state) };

            *zerop = gate[0] * zero_amp + gate[1] * one_amp;
            *onep = gate[2] * zero_amp + gate[3] * one_amp;
        });
    }

    pub fn apply_gate(&mut self, t: usize, gate: &QGate) {
        let n: usize = self.len() >> 1;

        for i in 0..n {
            let m: usize = (1 << t) - 1;

            let zero_state: usize = (i & m) | ((i & !m) << 1);
            let one_state: usize = zero_state | (1 << t);

            let zero_amp: c64 = self[zero_state];
            let one_amp: c64 = self[one_state];

            self[zero_state] = gate[0] * zero_amp + gate[1] * one_amp;
            self[one_state] = gate[2] * zero_amp + gate[3] * one_amp;
        }
    }

    pub fn apply_controlled_gate_par(&mut self, c: usize, t: usize, gate: &QGate) {
        let n: usize = self.len() >> 1;
        
        let reg_ptr = Pointer(self.as_mut_ptr());
        (0..n).into_par_iter().for_each(|i| {
            let m: usize = (1 << t) - 1;

            let zero_state: usize = (i & m) | ((i & !m) << 1);
            let one_state: usize = zero_state | (1 << t);

            let control_val_zero: usize = if ((1 << c) & zero_state) > 0 { 1 } else { 0 };
            let control_val_one: usize = if ((1 << c) & one_state) > 0 { 1 } else { 0 };

            let zero_amp: c64 = self[zero_state];
            let one_amp: c64 = self[one_state];

            if control_val_zero == 1 {
                let zerop = unsafe { &mut *{reg_ptr}.0.add(zero_state) };
                *zerop = gate[0] * zero_amp + gate[1] * one_amp;
            }

            if control_val_one == 1 {
                let onep = unsafe { &mut *{reg_ptr}.0.add(one_state) };
                *onep = gate[2] * zero_amp + gate[3] * one_amp;
            }
        });
    }

    pub fn apply_controlled_gate(&mut self, c: usize, t: usize, gate: &QGate) {
        let n: usize = self.len() >> 1;
        
        for i in 0..n {
            let m: usize = (1 << t) - 1;

            let zero_state: usize = (i & m) | ((i & !m) << 1);
            let one_state: usize = zero_state | (1 << t);

            let control_val_zero: usize = if ((1 << c) & zero_state) > 0 { 1 } else { 0 };
            let control_val_one: usize = if ((1 << c) & one_state) > 0 { 1 } else { 0 };

            let zero_amp: c64 = self[zero_state];
            let one_amp: c64 = self[one_state];

            if control_val_zero == 1 {
                self[zero_state] = gate[0] * zero_amp + gate[1] * one_amp;
            }

            if control_val_one == 1 {
                self[one_state] = gate[2] * zero_amp + gate[3] * one_amp;
            }
        }
    }

    pub fn _probabilities(&mut self) -> Vec<f64> {
        let n: usize = self.len();
        let mut probabilities: Vec<f64> = Vec::new();
        for i in 0..n {
            probabilities.push(self[i].abs());
        }
        probabilities
    }

    pub fn quantum_fourier_transform(&mut self, nqubits: usize) {

    let h: QGate = QGate::h();

    for j in 0..nqubits {
        for k in 0..j {
            let q: f64 = (1 << (j - k)) as f64;
            let theta: f64 = PI / q;
            let cp: QGate = QGate::cp(theta);
            self.apply_controlled_gate_par(j, k, &cp);
        }
        self.apply_gate_par(j, &h);
    }
    }
}

impl fmt::Display for QRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for state in self.iter() {
            write!(f, " {} ", state)?;
        }
        write!(f, "]")?;

        Ok(())
    }
}

impl Deref for QRegister {
    type Target = Vec<c64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for QRegister {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct QGate(c64x4);

impl QGate {
    pub fn new(z11: c64, z12: c64, z21: c64, z22: c64) -> Self {
        Self(c64x4::new(z11, z12, z21, z22))
    }
    pub fn cp(theta: f64) -> Self {
        QGate::new(
            c64::new(1.0, 0.0),
            c64::new(0.0, 0.0),
            c64::new(0.0, 0.0),
            c64::new(theta.cos(), theta.sin()),
        )
    }
    pub fn h() -> Self {
        QGate::new(
            c64::new(0.5_f64.sqrt(), 0.0),
            c64::new(0.5_f64.sqrt(), 0.0),
            c64::new(0.5_f64.sqrt(), 0.0),
            c64::new(-0.5_f64.sqrt(), 0.0),
        )
    }
}

impl Deref for QGate {
    type Target = c64x4;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for QGate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t{}\n{}\t{}", self[0], self[1], self[2], self[3])
    }
}
