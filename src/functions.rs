use std::f64::consts::E;

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
    0.5*x + 2.
}

pub fn sinusoidal(x:f64) -> f64 {
    x.sin()
}

pub fn absolute(x: f64) -> f64 {
    //((x-2.).abs()-2.).abs()
    x.abs()
}

pub fn mixed(x: f64) -> f64 {
    ((x-2.).abs()-2.).abs() + x.sin() + 0.05*x.powf(3.)
}

pub fn function_value(x:f64, func: Function) -> f64 {
    match func {
        Function::Poly1 => polynomial1(x),
        Function::Poly2 => polynomial2(x),
        Function::Linear => linear(x),
        Function::Sinusoidal => sinusoidal(x),
        Function::Absolute => absolute(x),
        Function::Mixed => mixed(x),
    }
}

pub fn weight(x: f64) -> f64 {
    E.powf(x * x)
}
