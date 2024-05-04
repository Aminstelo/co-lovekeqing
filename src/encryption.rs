use num_bigint::{BigInt,Sign};
use crate::cofunc_bigint::co as cofunc_co;
use crate::cofunc_bigint::aco as cofunc_aco;
use crate::rand_sha3::{get_func};
use crate::types::{Key,Data};

pub(crate) fn co(data:&Data, key:Key) -> Data {
    let mut ret = vec![];
    let hash_func = get_func(&key.0,key.1);
    for i in 0..data.len(){
        ret.push(cofunc_co(&data[i], &hash_func(&BigInt::from(i).to_bytes_be().1)))
    }
    ret
}

pub(crate) fn aco(cov :&Data,key:Key) -> Data {
    let mut ret = vec![];
    let hash_func = get_func(&key.0,key.1);
    for i in 0..cov.len(){
        ret.push(cofunc_aco(&cov[i], &hash_func(&BigInt::from(i).to_bytes_be().1)))
    }
    ret
}

#[test]
fn co_and_aco(){
    use crate::rand_sha3::{generate_random_hash_key};
    let original_bytes = vec![119, 120,378,92748,9219,1234];
    let mut new_bytes = vec![];
    for i in 0..original_bytes.len(){
        new_bytes.push(BigInt::from(original_bytes[i]));
    }
    let key = (generate_random_hash_key(),1145u64,1);
    let n1 = co(&new_bytes,key);
    let n2 = aco(&n1,key);
    println!("{:?},{:?}",n2,original_bytes);
}

#[test]
fn big_int_test(){
    // 原始的字节向量
    let original_bytes = vec![119, 120];
    // 从小端字节序的字节向量创建 BigInt
    let mut big_int = BigInt::from_bytes_be(Sign::Plus, &original_bytes);
    big_int -= 114514;
    big_int += 114514;
    // 将 BigInt 转换回小端字节序的字节向量
    let recovered_bytes: Vec<u8> = big_int.to_bytes_be().1;
    // 比较原始字节向量和恢复后的字节向量
    assert_eq!(original_bytes, recovered_bytes);
    println!("Original bytes: {:?}", original_bytes);
    println!("Recovered bytes: {:?}", recovered_bytes);
}