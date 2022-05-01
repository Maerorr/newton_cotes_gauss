use crate::{
    functions::{function_value, hermite_root, weight, hermite_weights},
    Function,
};

/// returns hermite-gauss integration, last argument is which weight the function uses
pub fn hermite_gauss(f: Function, n: usize, proper_weight: bool) -> f64 {
    let mut sum = 0.;

    for i in 0..n {
        match proper_weight {
            true => {
                sum += function_value(hermite_root(n, i), f, false) * hermite_weights(n, i);
            }
            false => {
                sum += function_value(hermite_root(n, i), f, false) * weight(hermite_root(n, i));
            }
        }
    }

    sum
}
