use std::mem::swap;

use crate::{Function, functions::{function_value, weight, hermite_root, hermite_poly, proper_weight}};

pub fn hermite_gauss(f: Function, n: usize) -> f64 {
    let mut sum = 0.;

    for i in 0..n {
        sum += function_value(hermite_root(n, i), f) * proper_weight(hermite_root(n, i), n);
    }

    sum
}
