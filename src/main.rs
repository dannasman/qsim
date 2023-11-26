mod circuit;
mod complex;
mod matrix;

use crate::circuit::*;
use crate::complex::*;
use crate::matrix::*;

fn main() {
    use std::time::Instant;
    let nqubits = 14;
    let n: usize = 1 << nqubits;
    let mut states: Vec<c64> = vec![c64::zero(); n];
    states[0] = c64::new(1.0, 0.0);
    let mut register: QRegister = QRegister::new(states);

    let now = Instant::now();

    register.quantum_fourier_transform(nqubits);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    //println!("{}", register);
}
