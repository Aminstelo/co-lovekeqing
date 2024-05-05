use num_bigint::{BigInt,Sign};
use crate::cofunc_bigint::{hash,hash_co,hash_aco};
use crate::types::{BigIntRef, Data, RawData, Key, ShardSize, HashModulus, VerificationHashModulus};
use crate::rand_sha3::hash as to_number_hash;
use crate::rand_sha3::get_func as get_key_hash_func;
use crate::processing::{slice_vec,data_to_vec,vec_u8_to_bigint};


pub(crate) fn data_hash(data : &RawData, chunk_size: ShardSize, co_func_d: VerificationHashModulus) -> Data{
    let mut ret = vec![];
    for (slice, _) in slice_vec(data,chunk_size as usize){
        ret.push(hash(&vec_u8_to_bigint(slice),&BigInt::from(co_func_d)))
    }
    ret
}

pub(crate) fn aco_hash(_data_hash:Data, key:Key, verification_hash_modulus:VerificationHashModulus) -> u64{
    // 使用模数为co_func_d 的 hash 函数的返回值值获取这个秘钥使用的验证用hash值
    let _get_key_hash_func = get_key_hash_func(&key.0,key.1);
    let mut need_hash_vec = vec![];
    for i in 0.._data_hash.len(){
        need_hash_vec.push(hash_aco(&_data_hash[i], &BigInt::from(verification_hash_modulus), &_get_key_hash_func(&BigInt::from(i).to_bytes_be().1)))
    }
    to_number_hash(&data_to_vec(need_hash_vec))
}

pub(crate) fn co_hash(cov :&Data, co_func_d: VerificationHashModulus) -> u64{
    // 使用加密数据获取加密数据对应秘钥和加密数据和对应co_func_d(模数)的验证用hash值
    let mut need_hash_vec = vec![];
    for cov_slice in cov{
        need_hash_vec.push(hash_co(&cov_slice,&BigInt::from(co_func_d)))
    }
    to_number_hash(&data_to_vec(need_hash_vec.clone()))
}

#[test]
fn co_and_aco_hash(){
    use crate::encryption::{aco, co};
    use crate::rand_sha3::{generate_random_hash_key};
    let original_bytes = vec![119, 120,3,117,225,30];
    let mut new_bytes = vec![];
    for i in 0..original_bytes.len(){
        new_bytes.push(BigInt::from(original_bytes[i]));
    }

    let verification_hash_modulus = 281u64;
    let key = (generate_random_hash_key(),1145u64,1);
    let n1 = co_hash(&co(&new_bytes, key), verification_hash_modulus);
    let n2 = aco_hash(data_hash(&original_bytes, key.2, verification_hash_modulus), key, verification_hash_modulus);
    println!("cov and d:{:?},RawDataHash and d and key:{:?}",n2,n1);
    assert_eq!(n1, n2); // 验证成功,cov加密前的文件和源文件一致
}