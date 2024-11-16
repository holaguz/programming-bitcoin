use crate::{
    ec::{ECurvePoint, EllipticCurve},
    finite_field::{FieldMod, FiniteField},
};
use lazy_static::lazy_static;
use num_bigint::BigUint;

type _SECPField = FiniteField<Secp256K1Mod>;

static SECP256K1_A: u32 = 0u32;
static SECP256K1_B: u32 = 7u32;

lazy_static! {
    /// The prime used on the secp256k1 curve.
    pub static ref SECP256K1_PRIME: BigUint = BigUint::parse_bytes(
        b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        16
    )
    .unwrap();

    /// The x coordinate of the generator on the secp256k1 curve.
    pub static ref SECP256K1_GX: _SECPField = _SECPField::new(
        BigUint::parse_bytes(
            b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
            16
        )
        .unwrap()
    );

    /// The y coordinate of the generator on the secp256k1 curve.
    pub static ref SECP256K1_GY: _SECPField = _SECPField::new(
        BigUint::parse_bytes(
            b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
            16
        )
        .unwrap()
    );

    /// The generator point on the secp256k1 curve.
    pub static ref SECP256K1_G: ECurvePoint<'static, FiniteField<Secp256K1Mod>> = SECP256K1.point_at(SECP256K1_GX.clone(), SECP256K1_GY.clone());

    /// The order of the generator on the secp256k1 curve.
    pub static ref SECP256K1_N: BigUint = BigUint::parse_bytes(
        b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
        16
    )
    .unwrap();

    /// The secp256k1 curve.
    pub static ref SECP256K1: EllipticCurve<FiniteField<Secp256K1Mod>> =
        EllipticCurve::new(SECP256K1_A, SECP256K1_B);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Secp256K1Mod;

impl FieldMod for Secp256K1Mod {
    fn modulus() -> BigUint {
        SECP256K1_PRIME.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ec::PointType;

    #[test]
    fn test_gen() {
        let g = SECP256K1_G.clone();
        match g.p {
            PointType::Point(c) => {
                assert_eq!(c.x, SECP256K1_GX.clone());
                assert_eq!(c.y, SECP256K1_GY.clone());
            }
            _ => panic!("Generator point not in the curve :/"),
        }
    }

    #[test]
    fn test_prime_field_order() {
        let g = SECP256K1_G.clone();
        let res = g * SECP256K1_N.clone();
        assert_eq!(res, SECP256K1.infinity());
    }
}
