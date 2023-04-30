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
pub fn list(last:String) -> String {
    if last.len() > 0 {
        if let Ok(request) = hex::decode(&last) {
            if let Ok(Some(r)) = decode::<pb::DataMap>(&request) {
                let mut s:String = "".to_owned();
                for (key, _) in r.map {
                    if s.len() == 0 {
                        s = "\"".to_owned() + &key + "\"";
                    }else{
                        s = s + ",\"" + &key + "\"";
                    }
                }
                return s;
            }
        }
    }
    return "".to_string();
}

#[wasm_bindgen]
pub fn register(last:String, address:String) -> String {
    let mut last_map = None;
    if last.len() == 0 {
        last_map = Some(pb::DataMap{
            map: HashMap::<String, pb::Data>::new(),
        });
    }else{
        if let Ok(request) = hex::decode(&last) {
            if let Ok(r) = decode(&request) {
                last_map = r;
            }
        }
    }

    match last_map {
        Some(lm) => {
            let rmap: pb::DataMap = _register(lm, &address);
            let rb = encode(CORE_DATA_MAP, &rmap).unwrap();
            let reply = hex::encode(rb);
            return reply;
        }
        _ => {
            return "".to_string()
        }
    }
}

#[test]
fn register_test() {
    let ret0 = register("".to_string(), "address0".to_string());
    let ret1 = register(ret0, "address1".to_string());
    let r = list(ret1);
    assert_eq!(r, "\"address0\",\"address1\"");
}

#[wasm_bindgen]
pub fn post(last:String, title:String, content:String) -> String {
    if last.len() == 0 {
        return "[\"".to_owned() + &title + "\",\"" + &content + "\"]";
    }else{
        return last + ",[\"" + &title + "\",\"" + &content + "\"]";
    }

    // let mut last_list = None;
    // if last.len() == 0 {
    //     last_list = Some(pb::DataList{
    //         list: vec![],
    //     });
    // }
    // if let Ok(request) = hex::decode(&last) {
    //     if let Ok(r) = decode(&request) {
    //         last_list = r;
    //     }
    // }
    // match last_list {
    //     Some(ll) => {
    //         let rlist: pb::DataList = _post(ll, title, content);
    //         let rb = encode(CORE_DATA_LIST, &rlist).unwrap();
    //         let reply = hex::encode(rb);
    //         return reply;
    //     }
    //     _ => {
    //         return "".to_string()
    //     }
    // }
}

#[test]
fn post_test() {
    let ret0 = post("".to_string(), "job_title_test0".to_string(), "job_content_test0".to_string());
    assert_eq!(&ret0, "[\"job_title_test0\",\"job_content_test0\"]");
    let ret1 = post(ret0, "job_title_test1".to_string(), "job_content_test1".to_string());
    assert_eq!(&ret1, "[\"job_title_test0\",\"job_content_test0\"],[\"job_title_test1\",\"job_content_test1\"]");
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
pub fn test_process(a:i32, s:String, b:i32) -> String {
    if s.len() == 0 {
        return "".to_string()
    }
    match hex::decode(s) {
        Ok(request) => {
            match decode(&request) {
                Ok(r) => match r {
                    Some(m) => {
                        let r: pb::DataMap = _test_process(m, a, b);
                        let rb = encode(CORE_DATA_MAP, &r).unwrap();
                        let reply = hex::encode(rb);
                        return reply;
                    }
                    _ => {
                        return "".to_string()
                    }
                }
                _ => {
                    return "".to_string()
                }
            }
        }
        _ => {
            return "".to_string()
        }
    }
}

#[wasm_bindgen]
pub fn test_list(a:i32, s:String, b:i32) -> String {
    if s.len() == 0 {
        return "".to_string()
    }
    match hex::decode(s) {
        Ok(request) => {
            match decode(&request) {
                Ok(r) => match r {
                    Some(list) => {
                        let rlist: pb::DataList = _test_list(list, a, b);
                        let rb = encode(CORE_DATA_LIST, &rlist).unwrap();
                        let reply = hex::encode(rb);
                        return reply;
                    }
                    _ => {
                        return "".to_string()
                    }
                }
                _ => {
                    return "".to_string()
                }
            }
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