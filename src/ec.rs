#![allow(dead_code)]

use std::ops::{Add, Div, Mul, Sub};

use num_bigint::BigUint;

pub trait FieldArithmetic:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    // + Rem<Output = Self>
    + Clone
    + PartialEq
    + From<u32>
{
}

impl FieldArithmetic for BigUint {}
impl FieldArithmetic for u32 {}
impl FieldArithmetic for u64 {}
impl FieldArithmetic for u128 {}

// Coordinates of a point on the curve
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Coordinates<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum PointType<T> {
    Invalid,
    Infinity,
    Point(Coordinates<T>),
}

// An elliptic curve defined by the equation y**2 = x**3 + Ax + B
#[derive(Debug, Eq, PartialEq)]
pub struct EllipticCurve<T> {
    a: T,
    b: T,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct ECurvePoint<'a, T> {
    curve: &'a EllipticCurve<T>,
    p: PointType<T>,
}

impl<'a, T> EllipticCurve<T>
where
    T: FieldArithmetic,
{
    pub fn new(a: impl Into<T>, b: impl Into<T>) -> Self {
        let a = a.into();
        let b = b.into();
        Self { a, b }
    }

    pub fn point_at(&'a self, x: impl Into<T>, y: impl Into<T>) -> ECurvePoint<'a, T> {
        let x = x.into();
        let y = y.into();
        match self.contains(&x, &y) {
            false => ECurvePoint::<'a, T> {
                curve: self,
                p: PointType::Invalid,
            },
            true => ECurvePoint::<'a, T> {
                curve: self,
                p: PointType::Point(Coordinates { x, y }),
            },
        }
    }

    pub fn infinity(&'a self) -> ECurvePoint<'a, T> {
        ECurvePoint::<'a, T> {
            curve: self,
            p: PointType::Infinity,
        }
    }

    pub fn contains(&self, x: &T, y: &T) -> bool {
        let lhs = y.clone() * y.clone();
        let x3 = x.clone() * x.clone() * x.clone();
        let rhs = x3 + self.a.clone() * x.clone() + self.b.clone();
        return lhs == rhs;
    }
}

impl<'a, T> Add for ECurvePoint<'a, T>
where
    T: FieldArithmetic,
{
    type Output = ECurvePoint<'a, T>;

    fn add(self, rhs: Self) -> Self::Output {
        // Points must be from the same curve
        assert!(
            self.curve == rhs.curve,
            "Cannot add points from different curves"
        );

        let (p, rhs) = match (&self.p, &rhs.p) {
            // Infinity is the additive identity
            (PointType::Infinity, _) => return rhs.clone(),
            (_, PointType::Infinity) => return self,

            // Invalid + anything = Invalid
            (PointType::Invalid, _) | (_, PointType::Invalid) => {
                return ECurvePoint {
                    curve: self.curve,
                    p: PointType::Invalid,
                }
            }
            (PointType::Point(p1), PointType::Point(p2)) => (p1, p2),
        };

        // 2. Points are additive inverses. The two points have the same x coord but different y.
        if p.x == rhs.x && p.y != rhs.y {
            return ECurvePoint {
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
                if p.y == 0u32.into() {
                    return ECurvePoint {
                        curve: self.curve,
                        p: PointType::Infinity,
                    };
                }
                let three: T = 3u32.into();
                let two: T = 2u32.into();
                (three * p.x.clone() * p.x.clone() + self.curve.a.clone()) / (two * p.y.clone())
            }
            false => (rhs.y.clone() - p.y.clone()) / (rhs.x.clone() - p.x.clone()),
        };

        let x = s.clone() * s.clone() - p.x.clone() - rhs.x.clone();
        let y = s * (p.x.clone() - x.clone()) - p.y.clone();

        ECurvePoint {
            curve: self.curve,
            p: PointType::Point(Coordinates { x, y }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finite_field::{FieldMod, FiniteField};

    mod finite_field {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct Field223Mod;

        impl FieldMod for Field223Mod {
            fn modulus() -> BigUint {
                223u32.into()
            }
        }

        type Field223 = FiniteField<Field223Mod>;

        fn test_curve() -> EllipticCurve<Field223> {
            EllipticCurve::new(0u32, 7u32)
        }

        #[test]
        fn test_contains() {
            let c = test_curve();

            let valid: Vec<(Field223, Field223)> = vec![
                (192u32.into(), 105u32.into()),
                (17u32.into(), 56u32.into()),
                (1u32.into(), 193u32.into()),
            ];
            let invalid: Vec<(Field223, Field223)> =
                vec![(200u32.into(), 119u32.into()), (42u32.into(), 99u32.into())];

            valid.iter().for_each(|(x, y)| {
                assert!(c.contains(x, y));
            });

            invalid.iter().for_each(|(x, y)| {
                assert!(!c.contains(x, y));
            });
        }

        #[test]
        fn test_add() {
            let c = test_curve();
            let a = c.point_at(192u32, 105u32);
            let b = c.point_at(17u32, 56u32);

            let result = c.point_at(170u32, 142u32);

            assert_eq!(a.clone() + b.clone(), result);
            assert_eq!(b + a, result);
        }
    }
}
