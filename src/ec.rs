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

        // 3. Points are the same point.
        if self == other {
            // Special case: If the y coord is 0, the tangent line is vertical since the elliptic
            // curve is symmetrical wrt. the x axis. This results on a point on the infinity.
            if self_coords.y == 0 {
                return Self::infinity();
            }

            let s = (3 * self_coords.x * self_coords.x + A) / (2 * self_coords.y);
            let x = s * s - 2 * self_coords.x;
            let y = s * (self_coords.x - x) - self_coords.y;
            return Self {
                p: Some(Coordinates { x, y }),
            };
        }

        // 4. The general case, where points are distinct, none is infinity and they are not additive inverses.
        let s = (other_coords.y - self_coords.y) / (other_coords.x - self_coords.x);
        let x = s * s - self_coords.x - other_coords.x;
        let y = s * (self_coords.x - x) - self_coords.y;

        return Self {
            p: Some(Coordinates { x, y }),
        };
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

    #[test]
    fn test_add_same() {
        let a = TEST_EC.point_at(-1, -1).unwrap();
        let res = TEST_EC.point_at(18, 77).unwrap();
        assert_eq!(res, a + a);

        let a = TEST_EC.point_at(-1, 1).unwrap();
        let res = TEST_EC.point_at(18, -77).unwrap();
        assert_eq!(res, a + a);
    }

    #[test]
    fn test_add() {
        let a = TEST_EC.point_at(-1, -1).unwrap();
        let b = TEST_EC.point_at(2, 5).unwrap();
        let res = TEST_EC.point_at(3, -7).unwrap();
        assert_eq!(res, a + b);
        assert_eq!(res, b + a);
    }
}
