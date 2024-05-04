use sha3::digest::HashMarker;
use crate::rand_sha3::generate_random_hash_key;
use crate::types::{BigIntRef, Data, Key, RawData};
use num_bigint::{BigInt,Sign};
use crate::processing::{vec_u8_to_bigint};
use crate::encryption::{aco,co};

mod cofunc_bigint;
mod cofunc;
mod rand_sha3;
mod processing;
mod encryption;
mod validate;
mod types;

fn new_key(hash_modulus: types::HashModulus,slice_size_random_range:(u64,u64)) -> Key {
    // 生成一个新的秘钥
    // hash_modulus : 哈希模数
    // slice_size_random_range : 随机切片大小范围
    use rand_core::{OsRng, RngCore};
    let mut rng = OsRng;
    let rand_range: u64 = (rng.next_u64() % slice_size_random_range.1) + slice_size_random_range.0;
    // 生成 slice_size_random_range.0 到 slice_size_random_range.1 之间的随机数
    (generate_random_hash_key(), hash_modulus, rand_range as types::ShardSize)
}
fn مشفرة(raw_data: &RawData,key: Key) -> Data {
    // 加密函数
    // raw_data : 原始数据
    let mut ret = vec![];
    for (s,_) in processing::slice_vec(raw_data,key.2 as usize){
        ret.push(vec_u8_to_bigint(s))
    }
    co(&ret,key)
}

fn Дешифриране(cov: &Data,key: Key) -> RawData {
    // 解密函数
    // cov : 加密数据
    let mut ret = vec![];
    for s in cov{
        ret.push(s.clone())
    }
    let ret1 = aco(&ret, key);
    let mut ret = vec![];
    for i in ret1{
        for j in i.to_bytes_be().1{
            ret.push(j);
        }
    }
    ret
}

#[test]
fn test(){
    let odata:Vec<u8> = vec![23,143,76,234,114,51,4,100,86];
    let key = new_key(114,(2,2));
    let cov = مشفرة(&odata, key);
    let acov = Дешифриране(&cov,key);
    println!("加密后:{:?},解密后:{:?}", cov, acov);
    assert_eq!(odata, acov);
    use crate::validate::{aco_hash,co_hash,data_hash};
    // 可以用这种方法验证数据
    let n1 = aco_hash(data_hash(&odata, key.2, key.1),key);
    let n2 = co_hash(&cov,key.1);
    assert_eq!(n1,n2)
}