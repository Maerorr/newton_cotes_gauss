use std::{
    f64::consts::{E, PI},
    vec,
};

use crate::Function;

pub fn polynomial1(x: f64) -> f64 {
    // 0.15x^2 - x - 1
    -1. + x * (-1. + x * 0.15)
}

pub fn polynomial2(x: f64) -> f64 {
    //0.07*x^4+0.3*x^3-0.2*x^2-x-1.
    -1. + x * (-1. + x * (-0.2 + x * (-0.3 + x * 0.07)))
}

pub fn linear(x: f64) -> f64 {
    0.5 * x + 2.
}

pub fn sinusoidal(x: f64) -> f64 {
    x.cos()
}

pub fn absolute(x: f64) -> f64 {
    //((x-2.).abs()-2.).abs()
    x.abs()
}

pub fn mixed(x: f64) -> f64 {
    ((x - 2.).abs() - 2.).abs() + x.sin() + 0.05 * x.powf(3.)
}

pub fn function_value(x: f64, func: Function) -> f64 {
    match func {
        Function::Poly1 => polynomial1(x)*weight(x),
        Function::Poly2 => polynomial2(x)*weight(x),
        Function::Linear => linear(x)*weight(x),
        Function::Sinusoidal => sinusoidal(x)*weight(x),
        Function::Absolute => absolute(x)*weight(x),
        Function::Mixed => mixed(x)*weight(x),
    }
}

/// returns e^(-x^2)
/// weight taken from the excerscise
pub fn weight(x: f64) -> f64 {
    E.powf(-x * x)
}

/// returns ath root of Hermite's polynomial of Nth degree
pub fn hermite_root(n: usize, a: usize) -> f64 {
    let arr: Vec<Vec<f64>> = vec![
        vec![],
        vec![0.],
        vec![-0.707106781187, 0.707106781187],
        vec![-1.224744871392, 0., 1.224744871392],
        vec![
            -1.650680123886,
            -0.524647623275,
            0.524647623275,
            1.650680123886,
        ],
        vec![
            -2.020182870456,
            -0.958572464614,
            0.,
            0.958572464614,
            2.020182870456,
        ],
        vec![
            -2.350604974,
            -1.335849074,
            -0.436077412,
            0.436077412,
            1.335849074,
            2.350604974,
        ],
    ];
    arr[n][a]
}

pub fn hermite_poly(x: f64, n: usize) -> f64 {
    if n == 2 {
        return 4. * x * x - 2.;
    } else if n == 3 {
        return 8. * x * x * x - 12. * x;
    } else if n == 4 {
        return 16. * x * x * x * x - 48. * x * x + 12.;
    } else if n == 5 {
        return 32. * x * x * x * x * x - 160. * x * x * x + 120. * x;
    } else if n == 6 {
        return 64. * x * x * x * x * x * x - 480. * x * x * x * x + 720. * x * x - 120.;
    } else {
        0.
    }
}

pub fn factorial(n: usize) -> f64 {
    if n == 0 {
        1.
    } else {
        n as f64 * factorial(n - 1)
    }
}

/// CALCULATES THE "PROPER" WEIGHT THAT RETURNS ACTUAL VALUES
///
/// !!!!!!!!!!!!!!!!!!!!!!
///
/// for n = 2, it divides by 0 and returns inf.
///
/// !!!!!!!!!!!!!!!!!!!!!!
///
pub fn proper_weight(x: f64, n: usize) -> f64 {
    let mut two: f64 = 2.;
    (two.powf((n - 1) as f64) * factorial(n) * PI.sqrt())
        / ((n * n) as f64 * (hermite_poly(x, n - 1).powf(2.)))
}
