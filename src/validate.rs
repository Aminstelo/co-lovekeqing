use num_bigint::{BigInt,Sign};
use crate::cofunc_bigint::{hash,hash_co,hash_aco};
use crate::types::{BigIntRef, Data, RawData,Key};
use crate::rand_sha3::hash as to_number_hash;
use crate::rand_sha3::get_func as get_key_hash_func;
use crate::processing::{slice_vec,data_to_vec};

fn data_hash(data : &RawData,chunk_size: u8,co_func_d:BigIntRef) -> Data{
    let mut ret = vec![];
    for (slice, _) in slice_vec(data,chunk_size as usize){
        ret.push(hash(&BigInt::from_bytes_be(Sign::Plus,slice),co_func_d))
    }
    ret
}

fn aco_hash(_data_hash:Data, co_func_d:BigIntRef, key:Key) -> u64{
    let _get_key_hash_func = get_key_hash_func(&key.0,key.1);
    let mut need_hash_vec = vec![];
    for i in 0.._data_hash.len(){
        need_hash_vec.push(hash_aco(&_data_hash[i], co_func_d, &_get_key_hash_func(&BigInt::from(i).to_bytes_be().1)))
    }
    to_number_hash(&data_to_vec(need_hash_vec))
}

fn co_hash(cov :&Data,co_func_d:BigIntRef) -> u64{
    let mut need_hash_vec = vec![];
    for cov_slice in cov{
        need_hash_vec.push(hash_co(&cov_slice,co_func_d))
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
    let key = (generate_random_hash_key(),1145u64);
    let d = &BigInt::from(56);
    let n1 = co_hash(&co(new_bytes,key),d);
    let n2 = aco_hash(data_hash(&original_bytes,1,d),d,key);
    println!("cov and d:{:?},RawDataHash and d and key:{:?}",n2,n1);
    assert_eq!(n1, n2);
}