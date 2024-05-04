use num_bigint::BigInt;

type HashFuncKey = [u8;32]; // cofunc_bigint.rs 中函数 f 参数生成使用的hash函数的Key
type HashModulus = u64; // cofunc_bigint.rs 中函数 f 参数生成使用的hash函数的模数
// type ShardSize = u8; // 数据加密时使用分片大小
pub type Key = (HashFuncKey,HashModulus); // HashFuncKey and HashModulus(不是验证hash函数的) (加密函数秘钥)
pub(crate) type BigIntRef<'a> = &'a BigInt; // cofunc_bigint.rs 中函数传入类型
pub type Data = Vec<BigInt>; // 加密函数参数原始数据
pub type RawData = Vec<u8>; // 未经过预处理的原始数据