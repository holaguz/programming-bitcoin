#![allow(dead_code)]
use std::fmt::Display;
use std::ops;

use crate::ec::FieldArithmetic;
use num_bigint::BigUint;

pub trait FieldMod: Clone + PartialEq {
    fn modulus() -> BigUint;
}
impl<F: FieldMod> FieldArithmetic for FiniteField<F> {}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FiniteField<F: FieldMod> {
    num: BigUint,
    _phantom: std::marker::PhantomData<F>,
}

impl<F: FieldMod> FiniteField<F> {
    pub fn new(num: impl Into<BigUint>) -> Self {
        let num = num.into();
        let modulus = F::modulus();
        assert!(
            num < modulus,
            "num {} not in field range 0 to {}",
            num,
            &modulus - 1u32
        );
        Self {
            num,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn exp(self, exponent: impl Into<BigUint>) -> Self {
        let exponent = exponent.into();
        // Modular exponentiation by squaring
        // Handle special cases first
        if exponent == 0u32.into() {
            return Self {
                num: 1u32.into(),
                _phantom: std::marker::PhantomData,
            };
        }

        if self.num == 0u32.into() {
            return Self {
                num: 0u32.into(),
                _phantom: std::marker::PhantomData,
            };
        }

        if self.num == 1u32.into() {
            return self;
        }

        let mut base = self.num;
        let mut exp = exponent;
        let mut result: BigUint = 1u32.into();
        let modulus = F::modulus();

        // Square and multiply algorithm
        while exp > 0u32.into() {
            if &exp % 2u32 == 1u32.into() {
                result = (&result * &base) % &modulus;
            }
            base = (&base * &base) % &modulus;
            exp >>= 1;
        }

        Self {
            num: result,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<F: FieldMod> ops::Add for FiniteField<F> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let modulus = F::modulus();
        Self {
            num: (self.num + other.num) % &modulus,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<F: FieldMod> ops::Sub for FiniteField<F> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let modulus = F::modulus();
        let mut num = self.num;
        while num < rhs.num {
            num += &modulus;
        }
        Self {
            num: (num - rhs.num) % &modulus,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<F: FieldMod> ops::Rem for FiniteField<F> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        let modulus = F::modulus();
        Self {
            num: (self.num % rhs.num) % &modulus,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<F: FieldMod> ops::Mul for FiniteField<F> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let modulus = F::modulus();
        Self {
            num: (self.num * other.num) % &modulus,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<F: FieldMod> ops::Div for FiniteField<F> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        // Using Fermat's Little Theorem:
        // In a finite field of prime order p, for any number a:
        // a^(p-1) ≡ 1 (mod p)
        // Therefore: a^(p-2) is the multiplicative inverse of a
        let exponent = F::modulus() - 2u32;
        self * other.exp(exponent)
    }
}

impl<F: FieldMod> Display for FiniteField<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FieldElement<{}>({})", F::modulus(), self.num)
    }
}

impl<F: FieldMod> From<u32> for FiniteField<F> {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct Field7;
    impl FieldMod for Field7 {
        fn modulus() -> BigUint {
            7u32.into()
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    struct Field13;
    impl FieldMod for Field13 {
        fn modulus() -> BigUint {
            13u32.into()
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    struct Field19;
    impl FieldMod for Field19 {
        fn modulus() -> BigUint {
            19u32.into()
        }
    }

    #[test]
    fn test_new_field_element() {
        let fe: FiniteField<Field7> = FiniteField::new(5u32);
        assert_eq!(fe.num, BigUint::from(5u32));
    }

    #[test]
    #[should_panic]
    fn test_new_invalid_field_element() {
        let _: FiniteField<Field7> = FiniteField::new(10u32);
    }

    #[test]
    fn test_fe_eq() {
        let a: FiniteField<Field7> = FiniteField::new(5u32);
        let b: FiniteField<Field7> = FiniteField::new(5u32);
        assert_eq!(a, b);
    }

    #[test]
    fn test_fe_neq() {
        let a: FiniteField<Field7> = FiniteField::new(5u32);
        let b: FiniteField<Field7> = FiniteField::new(6u32);
        assert_ne!(a, b);
    }

    #[test]
    fn test_fe_display() {
        let fe: FiniteField<Field7> = FiniteField::new(4u32);
        assert_eq!(format!("{}", fe), "FieldElement<7>(4)");
    }

    #[test]
    fn test_add() {
        let a: FiniteField<Field7> = FiniteField::new(4u32);
        let b: FiniteField<Field7> = FiniteField::new(4u32);
        let result = a + b;
        assert_eq!(result.num, BigUint::from(1u32));
    }

    #[test]
    fn test_mul() {
        let a: FiniteField<Field7> = FiniteField::new(4u32);
        let b: FiniteField<Field7> = FiniteField::new(4u32);
        let result = a * b;
        assert_eq!(result.num, BigUint::from(2u32));
    }

    #[test]
    fn test_exp() {
        let a: FiniteField<Field13> = FiniteField::new(3u32);
        let result: FiniteField<Field13> = FiniteField::new(1u32);
        assert_eq!(result, a.exp(3u32));

        assert_eq!(
            FiniteField::<Field13>::new(1u32),
            FiniteField::<Field13>::new(3u32).exp(0u32)
        );
        assert_eq!(
            FiniteField::<Field13>::new(1u32),
            FiniteField::<Field13>::new(0u32).exp(0u32)
        );
        assert_eq!(
            FiniteField::<Field13>::new(0u32),
            FiniteField::<Field13>::new(0u32).exp(3u32)
        );
    }

    #[test]
    fn test_div() {
        let a: FiniteField<Field19> = FiniteField::new(2u32);
        let b: FiniteField<Field19> = FiniteField::new(7u32);
        let result: FiniteField<Field19> = FiniteField::new(3u32);
        assert_eq!(a / b, result);
    }
}
