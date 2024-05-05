# cokeqing
 可验证加密函数-验证加密前的数据是否正确

## 用法
请查看以下例子
```rust
use cofunc::{new_key,مشفرة,Дешифриране}; // 加密解密相关
use cofunc::validate::{aco_hash,co_hash,data_hash}; // 验证相关
// 注意,这里的引用可能不是很准确,请自行分辨

let odata:Vec<u8> = vec![23,143,76,234,114,51,4,100,86]; // 原始数据
let key = new_key(114,(2,2)); // 创建一个秘钥
let cov = مشفرة(&odata, key); // 加密
let acov = Дешифриране(&cov,key); // 解密
println!("加密后:{:?},解密后:{:?}", cov, acov);
assert_eq!(odata, acov);
use crate::validate::{aco_hash,co_hash,data_hash};
// 可以用这种方法验证数据
let odata_hash = data_hash(&odata, key.2, key.1) // 原数据hash值
let n1 = aco_hash(odata_hash,key); // 使用原数据的hash值和秘钥获得用于验证的hash值
let n2 = co_hash(&cov,key.1); // 使用加密后的数据和对odata_hash进行hash运算的hash函数的模数获得用于验证的hash值
assert_eq!(n1,n2) // 加密前数据的hash值和源数据一致
```
![Alt Test](/images/wallhaven-9m9me8.jpg)
## 已知bug
### 可人为制造hash碰撞[1] (可规避)
在数学推导时,发现`cofunc`的`hash`函数的本质是取模,因此可以人为制造hash碰撞,但是幸运的是可以对加密者隐藏hash模数(这里引申出下面的问题).
### `types.rs`'s Key中包含验证使用的hash函数模数导致可人为制造hash碰撞 [2] (已解决)
这个问题是bug[1]引申出来的,只要加密者不知道验证使用的哈希函数模数,就基本不可能制造碰撞,至于如何在工程上实现这个信息差,请自己想办法(我已经想到了).
