mod ec;
mod finite_field;
mod secp256k1;

use secp256k1::{SECP256K1, SECP256K1_GX, SECP256K1_GY};

fn main() {
    let p = SECP256K1.point_at(SECP256K1_GX.clone(), SECP256K1_GY.clone());
    dbg!(p);
}
