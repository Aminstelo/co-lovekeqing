// 随机生成哈希函数,利用hash函数值作为秘钥,类似于基于hash算法生成随机数

use rand_core::{RngCore, OsRng};
use sha3::{Sha3_256, Digest};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use num_bigint::{BigInt};
use crate::types::{HashFuncKey,HashModulus};

pub(crate) fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub(crate) fn get_func(key : &HashFuncKey, hash_m : HashModulus) ->  impl Fn(&Vec<u8>) -> BigInt {
    let mut hasher = Sha3_256::new();
    hasher.update(key);
    let hash1 = hasher.finalize();
    let key_hash = hash1.to_vec();
    move |input| {
        let mut output = [0u8; 32];
        let mut hasher = Sha3_256::new();
        hasher.update(&key_hash);
        hasher.update(input);
        output.copy_from_slice(hasher.finalize().as_slice());
        BigInt::from(hash(&output) % hash_m)
    }
}

pub(crate) fn generate_random_hash_key() -> [u8;32]{
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

#[test]
fn main() {
    let input = b"Hello, world!";
    for i in 0..1{
        let hash_fn = get_func(&generate_random_hash_key(),1145);
        let output = hash_fn(&Vec::from(input));
        println!("{:?}", output);
    }
}
