// 数据预处理模块,这个模块可以有效平衡加密效率和加密安全
use crate::types::{Data, RawData};


pub(crate) fn slice_vec(data: &RawData, chunk_size: usize) -> Vec<(&[u8], usize)> {
    // Vec分片
    let mut result = Vec::new();
    let mut start = 0;

    while start < data.len() {
        let end = std::cmp::min(start + chunk_size, data.len());
        let slice = &data[start..end];
        result.push((slice, slice.len()));
        start = end;
    }
    result
}

pub(crate) fn data_to_vec(data:Data) -> Vec<Vec<u8>>{
    let mut ret = vec![];
    for i in data{
        ret.push(i.to_bytes_be().1)
    }
    ret
}

#[test]
fn test_slice_vec(){
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let slices = slice_vec(&data, 3);

    for (slice, len) in slices {
        println!("Slice: {:?}, Length: {}", slice, len);
    }
}

