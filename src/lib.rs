extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use bytes::{Buf, BufMut};
use std::str::from_utf8;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use prost::Message as ProtoMessage;

use sha2::{Sha256, Digest};

use ecies::{SecretKey, PublicKey};

use libsecp256k1::*;
use crate::{sign, Message};

mod pb {
    include!("./pb.rs");
}
include!("./core.rs");
include!("./tx.rs");
include!("./test.rs");

#[wasm_bindgen]
pub fn generate_account(phrase:String) -> String {
    let bs = phrase.as_bytes();
    let hash = keccak256(&bs.to_vec());

    let secret = "eth.".to_owned() + &hex::encode(&hash);
    
    let (_, _, pub_key, address) = get_secret(&secret).unwrap();
    let public = "eth.".to_owned() + &hex::encode(pub_key.serialize());

    // println!("{}", &secret);
    // println!("{}", &public);
    // println!("{}", &address);

    return ["ETH", &address, &secret, &public].join(",");

    // let mut mm = HashMap::<String, pb::Data>::new();
    // mm.insert("type".to_string(), pb::Data {
    //     bytes: encode_string("ETH".to_string()),
    // });
    // mm.insert("address".to_string(), pb::Data{
    //     bytes: encode_string(address),
    // });
    // mm.insert("private".to_string(), pb::Data{
    //     bytes: encode_string(secret),
    // });
    // mm.insert("public".to_string(), pb::Data{
    //     bytes: encode_string(public),
    // });
    // let nm = pb::DataMap{
    //     map: mm,
    // };
    // let rb = encode(CORE_DATA_MAP, &nm).unwrap();
    // let reply = hex::encode(rb);
    // return reply;
}

#[wasm_bindgen]
pub fn sign_transaction(s:String) -> String {
    if s.len() == 0 {
        return "".to_string()
    }
    match hex::decode(s) {
        Ok(request) => {
            match decode(&request) {
                Ok(r) => match r {
                    Some(m) => {
                        match _sign_tx(m) {
                            Ok(ret) => {
                                return ret;
                            }
                            Err(err) => {
                                println!("error: {:?}", err);
                                return "".to_string()
                            }
                        }
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

#[test]
fn data_test() {
    assert_eq!(CORE_DATA_INT32, get_type("int32"));
}

#[test]
fn generate_test(){
    generate_account("masterpassphrase".to_string());
}

#[test]
fn sign_test() {
    let blob = sign_transaction("410a390a0466726f6d12310a2f3e6574682e3078366461363861306335644141453037313541453662363246303066353438413243363938316332660a100a0367617312090a073e3230303030300aca070a077061796c6f616412be070abb07410ab7070a047061676512ae070aab07410a97070a0464617461128e070a8b073e3530346230333034313430303030303030383030653862393933353631633230366633323162303130303030623030323030303030613030316330303639366536343635373832653638373436643663353535343039303030333833303534303634343531333435363437353738306230303031303466373031303030303034313430303030303039353532346234666334323031306265656661666330333935626430363838633037363836323763356366356230396137386134333036623637393735326134373433376664663735326262396266386361633635656638363639386566393163306338393362626337646265356562643333646162353365336361383531633831333965646466313461303837373263313938616335316462623163383635383334393333353365626438363135326630626337633238616536313366373265343337326361323533343061366635633065613838326230353563616366343736626365306639383061663265356533376138363034626238306236643463633033323237613163666165336262326139353631366233306338626366653639343931613734346461313539646431306564353339336633623538323534613065636231373461633836653432393036326561613735393637323238356334626136383134303834373935383863313838373532376436656633646336613162386530353766633932616633623238613539386138666664326631386162636365333034333938363130656464333863613066333134373531623964363663376530383865326266613434366534653832366632646665653234623731353830643539623537366438356235386461333262323061373430383233336462376466626464333130663263373337616465643164633239346334343530366431363866323766363434636436333936623563643030663530346230313032316530333134303030303030303830306538623939333536316332303666333231623031303030306230303230303030306130303138303030303030303030303031303030303030613438313030303030303030363936653634363537383265363837343664366335353534303530303033383330353430363437353738306230303031303466373031303030303034313430303030303035303462303530363030303030303030303130303031303035303030303030303566303130303030303030300a0e0a046e616d6512060a043e6c69620a510a0673656372657412470a453e6574682e626565633965633631633137623034636239653461396237303137653734396639323833356532373433653935663934636465323138643636376231343130390a100a0873657175656e636512040a023e380a370a02746f12310a2f3e6574682e3078423561316432653932353231323439616230373933343130393543646165343443643439393837370a0f0a0576616c756512060a043e313030".to_string());
    println!("blob: {}", blob);
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
pub fn process(a:i32, s:String, b:i32) -> String {
    if s.len() == 0 {
        return "".to_string()
    }
    match hex::decode(s) {
        Ok(request) => {
            match decode(&request) {
                Ok(r) => match r {
                    Some(m) => {
                        let r: pb::DataMap = _process(m, a, b);
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
pub fn list(a:i32, s:String, b:i32) -> String {
    if s.len() == 0 {
        return "".to_string()
    }
    match hex::decode(s) {
        Ok(request) => {
            match decode(&request) {
                Ok(r) => match r {
                    Some(list) => {
                        let rlist: pb::DataList = _list(list, a, b);
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
    assert_eq!("".to_string(), process(1234, "".to_string(), 5678));
    assert_eq!("".to_string(), list(1234, "".to_string(), 5678));
}