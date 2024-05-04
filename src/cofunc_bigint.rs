use num_bigint::{BigInt};
use crate::types::BigIntRef;


pub(crate) fn hash(x: BigIntRef, d: BigIntRef) -> BigInt {
    x % d
}

pub(crate) fn co(x: BigIntRef, f: BigIntRef) -> BigInt {
    x + f
}

pub(crate) fn aco(x: BigIntRef, f: BigIntRef) -> BigInt {
    x - f
}

pub(crate) fn hash_co(cov: BigIntRef, d: BigIntRef) -> BigInt {
    hash(cov, d)
}

// fn mod_co_hash(f: i32, d: i32, x: i32) -> i32 {
//     co(hash(x, d), f) % d
// }

pub(crate) fn hash_aco(x_hash:BigIntRef,d:BigIntRef ,f:BigIntRef) -> BigInt {
    co(x_hash, f) % d
}
