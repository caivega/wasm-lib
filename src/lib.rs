extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use bytes::{Buf, BufMut};
use std::str::from_utf8;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use prost::Message as ProtoMessage;

use sha2::{Sha256, Digest};

mod pb {
    include!("./pb.rs");
}
include!("./core.rs");
include!("./test.rs");

#[test]
fn data_test() {
    assert_eq!(CORE_DATA_INT32, get_type("int32"));
}

#[wasm_bindgen]
pub fn list_register(last:String) -> String {
    let last_map = decode_map_from_string(&last);
    match last_map {
        Some(r) => {
            let mut s:String = "".to_owned();
            for (address, data) in r.map {
                if let Ok(Some(m)) = decode::<pb::DataMap>(&data.bytes) {
                    if let Ok(Some(name)) = get_string_from_map(&m, "name") {
                        if let Ok(Some(index)) = get_int64_from_map(&m, "index") {
                            if s.len() == 0 {
                                s = "[".to_owned() + &index.to_string() + ",\"" + &name + "\",\"" + &address + "\"]";
                            }else{
                                s = s + ",[" + &index.to_string() + ",\"" + &name + "\",\"" + &address + "\"]";
                            }
                        }
                    }
                }
            }
            return s;
        }
        _ => {
            return "".to_string()
        }
    }
}

#[wasm_bindgen]
pub fn register(last:String, name:String, address:String) -> String {
    let last_map = decode_map_from_string(&last);
    match last_map {
        Some(lm) => {
            let rmap: pb::DataMap = _register(lm, &name, &address);
            return encode_map_to_string(&rmap);
        }
        _ => {
            return "".to_string()
        }
    }
}

#[test]
fn register_test() {
    let ret0 = register("".to_string(), "name0".to_string(), "address0".to_string());
    let r0 = list_register(ret0.clone());
    assert_eq!(r0, "[0,\"name0\",\"address0\"]");
    let ret1 = register(ret0, "name1".to_string(), "address1".to_string());
    let r1 = list_register(ret1);
    assert_eq!(r1.len(), "[0,\"name0\",\"address0\"],[1,\"name1\",\"address1\"]".len());
}

#[wasm_bindgen]
pub fn list_post(last:String) -> String {
    let last_list = decode_list_from_string(&last);
    match last_list {
        Some(ll) => {
            let mut s:String = "".to_owned();
            for l in &ll.list {
                if let Ok(Some(m)) = decode::<pb::DataMap>(&l.bytes) {
                    if let Ok(Some(title)) = get_string_from_map(&m, "title") {
                        if let Ok(Some(content)) = get_string_from_map(&m, "content") {
                            if s.len() == 0 {
                                s = "[\"".to_owned() + &title + "\",\"" + &content + "\"]";
                            }else{
                                s = s + ",[\"" + &title + "\",\"" + &content + "\"]";
                            }
                        }
                    }
                }
            }
            return s;
        }
        _ => {
            return "".to_string()
        }
    }
}

#[wasm_bindgen]
pub fn post(last:String, title:String, content:String) -> String {
    let last_list = decode_list_from_string(&last);
    match last_list {
        Some(ll) => {
            let rlist: pb::DataList = _post(ll, title, content);
            return encode_list_to_string(&rlist);
        }
        _ => {
            return "".to_string()
        }
    }
}

#[wasm_bindgen]
pub fn list_apply(last:String) -> String {
    if let Some(ll) = decode_list_from_string(&last) {
        let mut s:String = "".to_owned();
        for i in 0..ll.list.len() {
            if let Ok(Some(la)) = get_map_from_list(&ll, i) {
                if let Ok(Some(list)) = get_list_from_map(&la, "list"){
                    for l in &list.list {
                        let info = hex::encode(&l.bytes);
                        if s.len() == 0 {
                            s = "[".to_owned() + &i.to_string() + ",\"" + &info + "\"]";
                        }else{
                            s = s + ",[" + &i.to_string() + ",\"" + &info + "\"]";
                        }
                    }
                }
            }
        }
        return s;
    }
    return "".to_string();
}

#[wasm_bindgen]
pub fn apply(last:String, index:i64, info:String) -> String {
    if let Some(ll) = decode_list_from_string(&last) {
        if let Ok(info_bytes) = hex::decode(info) {
            let rlist: pb::DataList = _apply(ll, index, info_bytes);
            return encode_list_to_string(&rlist);
        }
    }
    return "".to_string();
}

#[test]
fn post_test() {
    let ret0 = post("".to_string(), "job_title_test0".to_string(), "job_content_test0".to_string());
    let list0 = list_post(ret0.clone());
    assert_eq!(&list0, "[\"job_title_test0\",\"job_content_test0\"]");
    let ret1 = post(ret0, "job_title_test1".to_string(), "job_content_test1".to_string());
    let list1 = list_post(ret1.clone());
    assert_eq!(&list1, "[\"job_title_test0\",\"job_content_test0\"],[\"job_title_test1\",\"job_content_test1\"]");

    let info0 = hex::encode(b"test info0");
    let info1 = hex::encode(b"test info1");
    let ret2 = apply(ret1.clone(), 1i64, info0);
    let list2 = list_apply(ret2.clone());
    assert_eq!(&list2, "[1,\"7465737420696e666f30\"]");
    let ret3 = apply(ret2.clone(), 1i64, info1);
    let list3 = list_apply(ret3.clone());
    assert_eq!(&list3, "[1,\"7465737420696e666f30\"],[1,\"7465737420696e666f31\"]");
    let info2 = hex::encode(b"test info2");
    let ret4 = apply(ret3.clone(), 0i64, info2);
    let list4 = list_apply(ret4.clone());
    assert_eq!(&list4, "[0,\"7465737420696e666f32\"],[1,\"7465737420696e666f30\"],[1,\"7465737420696e666f31\"]");
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn test_i32_string(a: i32, b: String) -> i32 {
    return a + (b.len() as i32)
}

#[wasm_bindgen]
pub fn test_string_i32(a: String, b: i32) -> i32 {
    return (a.len() as i32) + b
}

#[wasm_bindgen]
pub fn test_string_i32_ret_string(a: String, b: i32) -> String {
    return format!("{}-{}", a, b)
}

#[wasm_bindgen]
pub fn test_i64_string_i32(a: i64, b: String, c: i32) -> i64 {
    return a + (b.len() as i64) + (c as i64)
}

#[wasm_bindgen]
pub fn test_string_i64_i32(a: String, b: i64, c: i32) -> i32 {
    return (a.len() as i32) + (b as i32) + c
}

#[wasm_bindgen]
pub fn test_string_i64_i32_ret_string(a: String, b: i64, c: i32) -> String {
    return format!("{}-{}-{}", a.len(), b, c)
}

#[wasm_bindgen]
pub fn test_bytes_i64_i32_ret_string(a: Vec<u8>, b: i64, c: i32) -> String {
    return format!("{}-{}-{}", a.len(), b, c)
}

#[wasm_bindgen]
pub fn test_string_string_ret_string(a: String, b: String) -> String {
    return format!("{}-{}", a, b)
}

#[wasm_bindgen]
pub fn test_f32_f64_ret_f64(a: f32, b: f64) -> f64 {
    return (a as f64) + b
}

#[wasm_bindgen]
pub fn test_f32_string_f64_ret_string(a: f32, b: String, c: f64) -> String {
    return format!("{}-{}-{}", a, b, c)
}

#[wasm_bindgen]
pub fn test_process(a:i32, last:String, b:i32) -> String {
    if last.len() == 0 {
        return "".to_string()
    }
    let last_map = decode_map_from_string(&last);
    match last_map {
        Some(m) => {
            let rmap: pb::DataMap = _test_process(m, a, b);
            return encode_map_to_string(&rmap);
        }
        _ => {
            return "".to_string()
        }
    }
}

#[wasm_bindgen]
pub fn test_list(a:i32, last:String, b:i32) -> String {
    if last.len() == 0 {
        return "".to_string()
    }
    let last_list = decode_list_from_string(&last);
    match last_list {
        Some(list) => {
            let rlist: pb::DataList = _test_list(list, a, b);
            return encode_list_to_string(&rlist);
        }
        _ => {
            return "".to_string()
        }
    }
}

#[wasm_bindgen]
pub fn call_winner(input: &str) -> i32 {
    let lines:[[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6]
    ];
    let split = input.split(",");
    let elements: Vec<&str> = split.collect();
    for i in 0..lines.len() {
        let [a, b, c] = lines[i];
        if elements[a].len() > 0 && elements[a] == elements[b] && elements[a] == elements[c] {
            match elements[a] {
                "X" => return 2,
                "O" => return 1,
                 _  => return 0,
            }
        }
    }
    return 0;
}

#[test]
fn add_test() {
    assert_eq!(1+1, add(1, 1));
}

#[test]
fn process_test() {
    assert_eq!("".to_string(), test_process(1234, "".to_string(), 5678));
    assert_eq!("".to_string(), test_list(1234, "".to_string(), 5678));
}