#![allow(dead_code)]

use std::ops;

// An elliptic curve defined by the equation y**2 = x**3 + Ax + B
#[derive(Debug, Eq, PartialEq)]
pub struct ECurve<const A: i32, const B: i32>;

// Coordinates of a point on the curve
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

// A point contained in the elliptic curve defined by A and B
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct EPoint<const A: i32, const B: i32> {
    // None represents the point at infinity
    p: Option<Coordinates>,
}

impl<const A: i32, const B: i32> EPoint<A, B> {
    // Constructor for the point at infinity
    pub const fn infinity() -> Self {
        Self { p: None }
    }

    // Constructor for a regular point
    pub const fn new(x: i32, y: i32) -> Self {
        Self {
            p: Some(Coordinates { x, y }),
        }
    }

    // Check if this is the point at infinity
    pub const fn is_infinity(&self) -> bool {
        self.p.is_none()
    }

    // Get the coordinates, or None for point at infinity
    pub const fn coords(&self) -> Option<Coordinates> {
        self.p
    }
}

impl<const A: i32, const B: i32> ECurve<A, B> {
    pub const fn new() -> Self {
        ECurve
    }

    pub const fn point_at(&self, x: i32, y: i32) -> Option<EPoint<A, B>> {
        if self.contains(x, y) {
            Some(EPoint {
                p: Some(Coordinates { x, y }),
            })
        } else {
            None
        }
    }

    pub const fn point_at_ifty(&self) -> EPoint<A, B> {
        EPoint::<A, B>::infinity()
    }

    pub const fn a(&self) -> i32 {
        A
    }

    pub const fn b(&self) -> i32 {
        B
    }

    pub const fn contains(&self, x: i32, y: i32) -> bool {
        y * y == x * x * x + A * x + B
    }
}

impl<const A: i32, const B: i32> ops::Add for EPoint<A, B> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        // ECurve addition is performed by intersecting a line between the two
        // points to add. There are three main cases: the intersects the curve
        // at either one, two or three points.
        // For two intersections, the line is either vertical (one point is at infinity) or tangent to the curve.

        // Handle common cases first:
        // 1. Either point is an infinity. This point is the identity point, i.e., A + Ifty = A

        if self.is_infinity() {
            return other;
        } else if other.is_infinity() {
            return self;
        }

        // At this point (lol) we know that neither point is at infinity.
        let self_coords = self.coords().unwrap();
        let other_coords = other.coords().unwrap();

        // 2. Points are additive inverses. The two points have the same x coord but different y.
        if self_coords.x == other_coords.x && self_coords.y != other_coords.y {
            return EPoint::<A, B>::infinity();
        }

        unimplemented!()
    }
}

pub const SECP256K1: ECurve<0, 7> = ECurve::new();

#[cfg(test)]
mod tests {
    use super::*;

    // This is the example elliptic curve used in the book.
    const TEST_EC: ECurve<5, 7> = ECurve::<5, 7>::new();

    #[test]
    fn test_ec_new() {
        _ = ECurve::<0, 7>::new();
    }

    #[test]
    fn test_contains() {
        let contained = TEST_EC.contains(-1, 1);
        let not_contained = TEST_EC.contains(-1, -2);

        assert!(contained);
        assert!(!not_contained);
    }

    #[test]
    fn test_point_at() {
        let exists = TEST_EC.point_at(-1, 1);
        let not_exists = TEST_EC.point_at(-1, -2);

        assert!(exists.is_some());
        assert!(not_exists.is_none());
    }

    #[test]
    fn test_add_ifty() {
        let a = TEST_EC.point_at(-1, 1).unwrap();
        let ifty = TEST_EC.point_at_ifty();

        assert_eq!(a + ifty, a);
        assert_eq!(ifty + a, a);
    }

    #[test]
    fn test_add_ident() {
        let a = TEST_EC.point_at(-1, 1).unwrap();
        let b = TEST_EC.point_at(-1, -1).unwrap();

        assert_eq!(a + b, TEST_EC.point_at_ifty());
        assert_eq!(b + a, TEST_EC.point_at_ifty());
    }
}
