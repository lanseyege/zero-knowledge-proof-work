use rug::{Assign, Integer};
use rug::integer::IsPrime;
use rug::rand::RandState;
use sha256::digest;

use rust_proof::inner_product_prover;

pub trait BulletProofImpl{

}

pub struct BulletProof{
    base_prover : BaseProver,

}
