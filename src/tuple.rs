use crate::approximate_eq;
use std::{fmt, ops};

#[derive(Default, Debug)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl ops::Add for Tuple {
    type Output = Tuple;
    fn add(self, _rhs: Self) -> Self {
        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
            w: self.w + _rhs.w,
        }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, _rhs: &Self) -> bool {
        approximate_eq(self.x, _rhs.x)
            && approximate_eq(self.y, _rhs.y)
            && approximate_eq(self.z, _rhs.z)
            && approximate_eq(self.w, _rhs.w)
    }
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{:.5}, {:.5}, {:.5}, {:.5}}}",
            self.x, self.y, self.z, self.w
        )
    }
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Tuple::new(x, y, z, 1.0)
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Tuple::new(x, y, z, 0.0)
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn w(&self) -> f64 {
        self.w
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::approximate_eq;
    #[test]
    fn a_tuple_with_w_1_is_a_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.w(), 1.0);
    }

    #[test]
    fn tuple_with_w_0_is_a_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.w(), 0.0);
    }

    #[test]
    fn point_creates_tuple_with_w_1() {
        let p = Tuple::point(4.0, -4.0, 3.0);
        let r = approximate_eq(p.w(), 1.0);
        assert!(r)
    }

    #[test]
    fn vector_creates_tuple_with_w_0() {
        let v = Tuple::vector(4.0, -4.0, 3.0);
        let r = approximate_eq(v.w(), 0.0);
        assert!(r)
    }

    #[test]
    fn tuples_are_equal() {
        let t0 = Tuple::new(4.3, -4.2, 3.1, 0.0);
        let t1 = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert_eq!(t0, t1);
    }

    #[test]
    fn tuples_are_not_equal() {
        let t0 = Tuple::default();
        let t1 = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert_ne!(t0, t1);
    }

    #[test]
    fn tuple_addition_works() {
        let t0 = Tuple::new(1.0, 1.0, 1.0, 1.0);
        let t1 = Tuple::new(2.0, 2.0, 2.0, 2.0);
        assert_eq!(
            t0 + t1,
            Tuple {
                x: 3.0,
                y: 3.0,
                z: 3.0,
                w: 3.0
            }
        );
    }
}
