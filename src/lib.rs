pub mod tuple;

const EPSILON: f64 = 0.00001;

/// Test the equivalence of two floating point numbers by comparing their difference. If the absolute value of their
/// difference is less than EPSILON, we will consider them equal
pub fn approximate_eq(n0: f64, n1: f64) -> bool {
    (n0 - n1).abs() < EPSILON
}
