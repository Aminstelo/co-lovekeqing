use num_bigint::BigInt;

pub(crate) type HashFuncKey = [u8;32]; // 生成 cofunc_bigint.rs 中的函数的f参数使用的hash函数的Key
pub(crate) type HashModulus = u64; // 生成 cofunc_bigint.rs 中函数 f 参数使用的hash函数的模数,The modulus of the hash function that generates the cofun key
pub(crate) type ShardSize = u8; // 数据加密时使用分片大小
pub type Key = (HashFuncKey,HashModulus,ShardSize); // HashFuncKey and HashModulus(不是验证hash函数的) and ShardSize (加密函数秘钥)
pub(crate) type BigIntRef<'a> = &'a BigInt; // cofunc_bigint.rs 中函数传入类型
pub type Data = Vec<BigInt>; // 加密函数参数原始数据
pub type RawData = Vec<u8>; // 未经过预处理的原始数据
pub type VerificationHashModulus = u64; // validate.rs 中hash函数的模数