use crate::{
    functions::{self, function_value, hermite_poly, hermite_root, proper_weight, weight},
    Function,
};

/// returns hermite-gauss integration, last argument is which weight the function uses
pub fn hermite_gauss(f: Function, n: usize, proper_weight: bool) -> f64 {
    let mut sum = 0.;

    for i in 0..n {
        match proper_weight {
            true => {
                sum += function_value(hermite_root(n, i), f)
                    * functions::proper_weight(hermite_root(n, i), n);
            }
            false => {
                sum += function_value(hermite_root(n, i), f) * weight(hermite_root(n, i));
            }
        }
    }

    sum
}
