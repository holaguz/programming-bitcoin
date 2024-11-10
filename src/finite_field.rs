#![allow(dead_code)]
use std::fmt::Display;
use std::ops;

// TODO: Can FieldElement be generic over prime to avoid runtime checking the value of prime when
// doing an operation?

// A FieldElement is an element in a finite field.
#[derive(Debug, Eq, PartialEq)]
pub struct FieldElement {
    num: u128,
    prime: u128,
}

impl FieldElement {
    pub fn new(num: u128, prime: u128) -> Self {
        assert!(
            num < prime,
            "num {} not in field range 0 to {}",
            num,
            prime - 1
        );
        Self { num, prime }
    }

    // Modular exponentiation by squaring
    // TODO: Handle negative exponents
    pub fn exp(self, mut exponent: i128) -> Self {
        // Handle common exponent cases
        match exponent {
            0 => {
                return Self {
                    num: 1,
                    prime: self.prime,
                }
            }
            i128::MIN..0 => unimplemented!(),
            _ => (),
        }

        match self.num {
            0 => {
                return Self {
                    num: 0,
                    prime: self.prime,
                }
            }
            1 => {
                return Self {
                    num: self.num,
                    prime: self.prime,
                }
            }
            _ => (),
        }

        // Handle the positive exponent case
        let mut x = self.num;
        let mut y: u128 = 1;
        while exponent > 1 {
            if exponent % 2 == 1 {
                // exponent is odd
                y = (x * y) % self.prime;
                exponent = exponent - 1;
            }
            x = (x * x) % self.prime;
            exponent = exponent / 2;
        }

        Self {
            num: (x * y) % self.prime,
            prime: self.prime,
        }
    }
}

impl ops::Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        assert_eq!(
            self.prime, other.prime,
            "Cannot add elements of different fields (lhs: {}, rhs: {})",
            self.prime, other.prime
        );
        Self {
            num: (self.num + other.num) % self.prime,
            prime: self.prime,
        }
    }
}

impl ops::Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        assert_eq!(
            self.prime, other.prime,
            "Cannot multiply elements of different fields (lhs: {}, rhs: {})",
            self.prime, other.prime
        );

        Self {
            num: (self.num * other.num) % self.prime,
            prime: self.prime,
        }
    }
}

impl ops::Div for FieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        assert_eq!(
            self.prime, other.prime,
            "Cannot multiply elements of different fields (lhs: {}, rhs: {})",
            self.prime, other.prime
        );

        // a/b = a * b**-1
        // By Fermat's Little Theorem, b**(p-1) = 1 for p prime
        // b**-1 = b**-1 * 1 = b**-1 * b**(p-1) = b**(p-2)
        // This means that for example in F_19, b**18 = 1 and b**17 = b*-1 for all b > 0
        // a/b = a * b**17
        let p = other.prime;
        self * other.exp((p - 2) as i128)
    }
}

impl Display for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FieldElement<{}>({})", self.prime, self.num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_field_element() {
        let fe = FieldElement::new(5, 7);
        assert_eq!(fe.num, 5);
        assert_eq!(fe.prime, 7);
    }

    #[test]
    #[should_panic]
    fn test_new_invalid_field_element() {
        FieldElement::new(10, 10);
    }

    #[test]
    fn test_fe_eq() {
        let a = FieldElement::new(5, 10);
        let b = FieldElement::new(5, 10);
        assert_eq!(a, b);
    }

    #[test]
    fn test_fe_neq() {
        let a = FieldElement::new(5, 10);
        let b = FieldElement::new(6, 10);
        let c = FieldElement::new(5, 9);
        assert_ne!(a, b);
        assert_ne!(a, b);
        assert_ne!(b, c);
    }

    #[test]
    fn test_fe_display() {
        let fe = FieldElement::new(4, 7);
        assert_eq!(format!("{}", fe), "FieldElement<7>(4)");
    }

    #[test]
    fn test_add() {
        let a = FieldElement::new(4, 7);
        let b = FieldElement::new(4, 7);
        let result = a + b;
        assert_eq!(1, result.num);
    }

    #[test]
    #[should_panic]
    fn test_add_different_field() {
        let a = FieldElement::new(4, 7);
        let b = FieldElement::new(4, 8);
        _ = a + b;
    }

    #[test]
    fn test_mul() {
        let a = FieldElement::new(4, 7);
        let b = FieldElement::new(4, 7);
        let result = a * b;
        assert_eq!(2, result.num);
    }

    #[test]
    #[should_panic]
    fn test_mul_different_field() {
        let a = FieldElement::new(4, 7);
        let b = FieldElement::new(4, 8);
        _ = a + b;
    }

    #[test]
    fn test_exp() {
        let a = FieldElement::new(3, 13);
        let result = FieldElement::new(1, 13);
        assert_eq!(result, a.exp(3));

        let a = FieldElement::new(445, 1234);
        let exponent = 1 << 127 - 63;
        let result = FieldElement::new(703, 1234);
        assert_eq!(result, a.exp(exponent));

        assert_eq!(FieldElement::new(1, 13), FieldElement::new(3, 13).exp(0));
        assert_eq!(FieldElement::new(1, 13), FieldElement::new(0, 13).exp(0));
        assert_eq!(FieldElement::new(0, 13), FieldElement::new(0, 13).exp(3));
    }

    #[test]
    fn test_div() {
        let a = FieldElement::new(2, 19);
        let b = FieldElement::new(7, 19);
        let result = FieldElement::new(3, 19);

        assert_eq!(a / b, result);
    }

    #[test]
    #[should_panic]
    fn test_div_different_field() {
        let a = FieldElement::new(2, 19);
        let b = FieldElement::new(7, 69);
        _ = a + b;
    }
}
