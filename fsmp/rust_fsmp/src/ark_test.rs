use ark_ff::{
    biginteger::{BigInt, BigInteger, BigInteger256},
    fields::{FftField, Field, Fp6Config, PrimeField},
    One, UniformRand, Zero,
};
use ark_std::{
    cmp::Ordering,
    ops::{AddAssign, MulAssign},
};

use ark_bn254::{Fq, Fq12, Fq2, Fq6, Fq6Config, Fr};

fn test_fq_repr_from() {
    assert_eq!(BigInteger256::from(100u64), BigInt::new([100, 0, 0, 0]));
}


fn main() {
    println!("begin ... ");

    test_fq_repr_from();

    println!("end ... ");
}
