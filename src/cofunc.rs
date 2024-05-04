// use std::num::Wrapping;

// type Wu8 = Wrapping<u8>;
type Wu8 = u8;
fn hash(x: Wu8, d: Wu8) -> Wu8{
    x % d
}

fn co(x: Wu8, f: Wu8) -> Wu8{
    // x : 原文 , f : 秘钥
    x + f
}

fn aco(x: Wu8, f: Wu8) -> Wu8{
    x - f
}

fn hash_co(cov: Wu8, d: Wu8) -> Wu8{
    // cov : 加密的数据 ,d : 模数
    hash(cov, d)
}

// 注意 : co(hash(x,d),f)%d == hash_co(co(x,f),d)
