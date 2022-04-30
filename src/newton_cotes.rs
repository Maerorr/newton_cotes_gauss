use std::mem::swap;

use crate::{Function, functions::{function_value, weight}};

fn newton_cotes_iteration(f: Function, a: f64, b: f64) -> f64 {
    let h = (b - a) / 2. as f64;
    let mut sum = 0.;
    sum += function_value(a, f) * weight(a);
    sum += 4.*function_value(a+h, f) * weight(a+h);
    sum += function_value(b, f) * weight(a+h);

    sum * h / 3.
}

/// Returns the value of the Newton-Cotes integration formula for the given function with given precision.
/// * f - chosen function from the Function enum
/// * (a, b) - integration left and right bound
/// * eps - precision calculated as difference between next iterations.
pub fn newton_cotes(f: Function, mut a: f64, mut b: f64, eps: f64) -> (f64, i32) {
    if a > b {
        swap(&mut a, &mut b);
    }

    // first iteration
    let mut value1 = newton_cotes_iteration(f, a, b);

    // second iteration
    let mut step = (b-a)/2.;
    let mut value2 = newton_cotes_iteration(f, a, a+step);
    value2 += newton_cotes_iteration(f, a+step, b);

    // check for precision
    if (value2 - value1).abs() < eps {
       return (value2, 2);
    } else {
        // all next iterations
        let mut iteration = 3;
        // loop based on precision
        while (value2 - value1).abs() > eps {
            value1 = value2;
            value2 = 0.;
            step = (b-a) / iteration as f64;
            for i in 0..iteration {
                value2 += newton_cotes_iteration(f, a+(i as f64)*step, a+(i as f64+1.)*step);
            }
            iteration += 1;
        }

        (value2, iteration)
    }
}
