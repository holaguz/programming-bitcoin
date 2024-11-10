#![allow(dead_code)]

use crate::finite_field_generic::FiniteField;
use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum PointType<T: FiniteField> {
    Invalid,
    Infinity,
    Point(Coordinates<T>),
}

// An elliptic curve defined by the equation y**2 = x**3 + Ax + B
#[derive(Debug, Eq, PartialEq)]
pub struct ECurve<T: FiniteField> {
    a: T,
    b: T,
}

// Coordinates of a point on the curve
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Coordinates<T: FiniteField> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct ECPoint<'a, T: FiniteField> {
    curve: &'a ECurve<T>,
    p: PointType<T>,
}

impl<'a, T: FiniteField> ECurve<T> {
    pub const fn new(a: T, b: T) -> Self {
        Self { a, b }
    }

    pub fn point_at(&'a self, x: T, y: T) -> ECPoint<'a, T> {
        match self.contains(x, y) {
            false => ECPoint::<'a, T> {
                curve: self,
                p: PointType::Invalid,
            },
            true => ECPoint::<'a, T> {
                curve: self,
                p: PointType::Point(Coordinates { x, y }),
            },
        }
    }

    pub fn infinity(&'a self) -> ECPoint<'a, T> {
        ECPoint::<'a, T> {
            curve: self,
            p: PointType::Infinity,
        }
    }

    pub fn contains(&self, x: T, y: T) -> bool {
        y * y == x * x * x + self.a * x + self.b
    }
}

impl<T: FiniteField> Add for ECPoint<'_, T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        // Points must be from the same curve
        assert!(
            self.curve == rhs.curve,
            "Cannot add points from different curves"
        );

        let (p, rhs) = match (self.p, rhs.p) {
            // Infinity is the additive identity
            (PointType::Infinity, _) => return rhs,
            (_, PointType::Infinity) => return self,

            // Invalid + anything = Invalid
            (PointType::Invalid, _) | (_, PointType::Invalid) => {
                return ECPoint {
                    curve: self.curve,
                    p: PointType::Invalid,
                }
            }
            (PointType::Point(p1), PointType::Point(p2)) => (p1, p2),
        };

        // 2. Points are additive inverses. The two points have the same x coord but different y.
        if p.x == rhs.x && p.y != rhs.y {
            return ECPoint {
                curve: self.curve,
                p: PointType::Infinity,
            };
        }

        // 3. Either the points are the same point (P1 = P2) or are different (P1 != P2)
        // The only difference between the two cases is how we calculate the slope. For P1 == P2,
        // the line is tangent to the curve. For P1 != P2 the line intersects the curve at both
        // points. Furthermore, when P1 == P2 and P1.y == 0 the tangent line is vertical and the
        // resulting point lies at infinity.
        let s = match p == rhs {
            // 3.1. Points are the same point.
            true => {
                // Special case: If the y coord is 0, the tangent line is vertical since the elliptic
                // curve is symmetrical wrt. the x axis. This results on a point on the infinity.
                if p.y == T::from_i32(0).unwrap() {
                    return ECPoint {
                        curve: self.curve,
                        p: PointType::Infinity,
                    };
                }
                let three = T::from_i32(3).unwrap();
                let two = T::from_i32(2).unwrap();
                (three * p.x * p.x + self.curve.a) / (two * p.y)
            }
            false => (rhs.y - p.y) / (rhs.x - p.x),
        };

        let x = s * s - p.x - rhs.x;
        let y = s * (p.x - x) - p.y;

        ECPoint {
            curve: self.curve,
            p: PointType::Point(Coordinates { x, y }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod i32_field {
        use super::*;
        #[test]
        fn test_contains() {
            let curve = ECurve::new(5_i32, 7_i32);
            let contained = curve.contains(-1, 1);
            let not_contained = curve.contains(-1, -2);

            assert!(contained);
            assert!(!not_contained);
        }

        #[test]
        fn test_point_at() {
            let curve = ECurve::new(5_i32, 7_i32);
            let exists = curve.point_at(-1, 1);
            let not_exists = curve.point_at(-1, -2);

            assert_eq!(exists.p, PointType::Point(Coordinates { x: -1, y: 1 }));
            assert_eq!(not_exists.p, PointType::Invalid);
        }

        #[test]
        fn test_add_infinity() {
            let curve = ECurve::new(5_i32, 7_i32);
            let a = curve.point_at(-1, 1);
            let ifty = curve.infinity();

            assert_eq!(a + ifty, a);
            assert_eq!(ifty + a, a);
        }

        #[test]
        fn test_add_inverse() {
            let curve = ECurve::new(5_i32, 7_i32);
            let a = curve.point_at(-1, 1);
            let b = curve.point_at(-1, -1);

            assert_eq!(a + b, curve.infinity());
            assert_eq!(b + a, curve.infinity());
        }

        #[test]
        fn test_add_same() {
            let curve = ECurve::new(5_i32, 7_i32);
            let a = curve.point_at(-1, -1);
            let res = curve.point_at(18, 77);
            assert_eq!(res, a + a);

            let a = curve.point_at(-1, 1);
            let res = curve.point_at(18, -77);
            assert_eq!(res, a + a);
        }

        #[test]
        fn test_add_same_at_y0() {
            let curve = ECurve::new(1_i32, 10_i32);
            let p = curve.point_at(-2, 0);
            assert_eq!(curve.infinity(), p + p);
        }

        #[test]
        fn test_add() {
            let curve = ECurve::new(5_i32, 7_i32);
            let a = curve.point_at(-1, -1);
            let b = curve.point_at(2, 5);
            let res = curve.point_at(3, -7);
            assert_eq!(res, a + b);
            assert_eq!(res, b + a);
        }
    }

    mod prime_field {
        use super::*;
        use crate::finite_field_generic::FieldElement;
        type FE = FieldElement<i32>;

        #[test]
        fn test_contains() {
            let a = FE::new(0, 103).unwrap();
            let b = FE::new(7, 103).unwrap();
            let curve = ECurve::new(a, b);

            let x = FE::new(17, 103).unwrap();
            let y = FE::new(64, 103).unwrap();
            let contained = curve.contains(x, y);

            let x = FE::new(0, 103).unwrap();
            let y = FE::new(1, 103).unwrap();
            let not_contained = curve.contains(x, y);

            assert!(contained);
            assert!(!not_contained);

            // The same point (17,64) should not be contained in the curve over the field of the
            // integers
            let curve = ECurve::new(0, 7);
            let not_contained = curve.contains(17, 64);
            assert!(!not_contained);
        }

        //     #[test]
        //     fn test_point_at() {
        //         let curve = ECurve::new(5_i32, 7_i32);
        //         let exists = curve.point_at(-1, 1);
        //         let not_exists = curve.point_at(-1, -2);
        //
        //         assert_eq!(exists.p, PointType::Point(Coordinates { x: -1, y: 1 }));
        //         assert_eq!(not_exists.p, PointType::Invalid);
        //     }
        //
        //     #[test]
        //     fn test_add_infinity() {
        //         let curve = ECurve::new(5_i32, 7_i32);
        //         let a = curve.point_at(-1, 1);
        //         let ifty = curve.infinity();
        //
        //         assert_eq!(a + ifty, a);
        //         assert_eq!(ifty + a, a);
        //     }
        //
        //     #[test]
        //     fn test_add_inverse() {
        //         let curve = ECurve::new(5_i32, 7_i32);
        //         let a = curve.point_at(-1, 1);
        //         let b = curve.point_at(-1, -1);
        //
        //         assert_eq!(a + b, curve.infinity());
        //         assert_eq!(b + a, curve.infinity());
        //     }
        //
        //     #[test]
        //     fn test_add_same() {
        //         let curve = ECurve::new(5_i32, 7_i32);
        //         let a = curve.point_at(-1, -1);
        //         let res = curve.point_at(18, 77);
        //         assert_eq!(res, a + a);
        //
        //         let a = curve.point_at(-1, 1);
        //         let res = curve.point_at(18, -77);
        //         assert_eq!(res, a + a);
        //     }
        //
        //     #[test]
        //     fn test_add_same_at_y0() {
        //         let curve = ECurve::new(1_i32, 10_i32);
        //         let p = curve.point_at(-2, 0);
        //         assert_eq!(curve.infinity(), p + p);
        //     }
        //
        //     #[test]
        //     fn test_add() {
        //         let curve = ECurve::new(5_i32, 7_i32);
        //         let a = curve.point_at(-1, -1);
        //         let b = curve.point_at(2, 5);
        //         let res = curve.point_at(3, -7);
        //         assert_eq!(res, a + b);
        //         assert_eq!(res, b + a);
        //     }
    }
}
