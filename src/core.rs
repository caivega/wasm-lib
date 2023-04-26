use tiny_keccak::{Hasher, Keccak};

const CORE_DATA:u8  = 11;

const CORE_DATA_NULL:u8    = 50;
const CORE_DATA_BOOLEAN:u8 = 51;

const CORE_DATA_INT8:u8  = 52;
const CORE_DATA_INT16:u8 = 53;
const CORE_DATA_INT32:u8 = 54;
const CORE_DATA_INT64:u8 = 55;

const CORE_DATA_UINT8:u8  = 56;
const CORE_DATA_UINT16:u8 = 57;
const CORE_DATA_UINT32:u8 = 58;
const CORE_DATA_UINT64:u8 = 59;

const CORE_DATA_FLOAT32:u8 = 60;
const CORE_DATA_FLOAT64:u8 = 61;

const CORE_DATA_STRING:u8 = 62;
const CORE_DATA_BYTES:u8  = 63;

const CORE_DATA_LIST:u8 = 64;
const CORE_DATA_MAP:u8  = 65;

const CORE_TRANSACTION:u8 = 101;

const CORE_CONTRACT_INFO:u8 = 142;
const CORE_PAGE_INFO:u8 = 147;
const CORE_CODE_INFO:u8 = 148;

// KeyType
const ETH:u16 = 2;

fn get_info(meta: u8) -> &'static str {
    match meta {
        CORE_DATA => {
            return "data";
        }
        CORE_DATA_NULL => {
            return "data_null";
        }
        CORE_DATA_BOOLEAN => {
            return "data_boolean";
        }
        CORE_DATA_INT8 => {
            return "data_int8";
        }
        CORE_DATA_INT16 => {
            return "data_int16";
        }
        CORE_DATA_INT32 => {
            return "data_int32";
        }
        CORE_DATA_INT64 => {
            return "data_int64";
        }
        CORE_DATA_UINT8 => {
            return "data_uint8";
        }
        CORE_DATA_UINT16 => {
            return "data_uint16";
        }
        CORE_DATA_UINT32 => {
            return "data_uint32";
        }
        CORE_DATA_UINT64 => {
            return "data_uint64";
        }
        CORE_DATA_FLOAT32 => {
            return "data_float32";
        }
        CORE_DATA_FLOAT64 => {
            return "data_float64";
        }
        CORE_DATA_STRING => {
            return "data_string";
        }
        CORE_DATA_BYTES => {
            return "data_bytes";
        }
        CORE_DATA_LIST => {
            return "data_list";
        }
        CORE_DATA_MAP => {
            return "data_map";
        }
        _ => {
            return "";
        }
    }
}

fn get_type(t: &str) -> u8 {
    match t {
        "boolean" => {
            return CORE_DATA_BOOLEAN;
        }
        "int8" => {
            return CORE_DATA_INT8;
        }
        "int16" => {
            return CORE_DATA_INT16;
        }
        "int32" => {
            return CORE_DATA_INT32;
        }
        "int64" => {
            return CORE_DATA_INT64;
        }
        "uint8" => {
            return CORE_DATA_UINT8;
        }
        "uint16" => {
            return CORE_DATA_UINT16;
        }
        "uint32" => {
            return CORE_DATA_UINT32;
        }
        "uint64" => {
            return CORE_DATA_UINT64;
        }
        "float32" => {
            return CORE_DATA_FLOAT32;
        }
        "float64" => {
            return CORE_DATA_FLOAT64;
        }
        "string" => {
            return CORE_DATA_STRING;
        }
        "bytes" => {
            return CORE_DATA_BYTES;
        }
        "list" => {
            return CORE_DATA_LIST;
        }
        "map" => {
            return CORE_DATA_MAP;
        }
        _ => {
            return 0;
        }
    }
}

fn get_from_string(meta: u8, s: &str) -> Result<Vec<u8>, Error> {
    match meta {
        CORE_DATA_BOOLEAN => {
            match s.parse::<bool>() {
                Ok(b) => {
                    return Ok(encode_boolean(b));
                }
                Err(err) => {
                    return Err(Error::new(ErrorKind::InvalidData, err))
                }
            }
        }
        CORE_DATA_INT8 => {
            match s.parse::<i8>() {
                Ok(v) => {
                    return Ok(encode_i8(v));
                }
                Err(err) => {
                    return Err(Error::new(ErrorKind::InvalidData, err))
                }
            }
        }
        CORE_DATA_INT16 => {
            match s.parse::<i16>() {
                Ok(v) => {
                    return Ok(encode_i16(v));
                }
                Err(err) => {
                    return Err(Error::new(ErrorKind::InvalidData, err))
                }
            }
        }
        CORE_DATA_INT32 => {
            match s.parse::<i32>() {
                Ok(v) => {
                    return Ok(encode_i32(v));
                }
                Err(err) => {
                    return Err(Error::new(ErrorKind::InvalidData, err))
                }
            }
        }
        CORE_DATA_INT64 => {
            match s.parse::<i64>() {
                Ok(v) => {
                    return Ok(encode_i64(v));
                }
                Err(err) => {
                    return Err(Error::new(ErrorKind::InvalidData, err))
                }
            }
        }
        CORE_DATA_UINT8 => {
            match s.parse::<u8>() {
                Ok(v) => {
                    return Ok(encode_u8(v));
                }
                Err(err) => {
                    return Err(Error::new(ErrorKind::InvalidData, err))
                }
            }
        }
        CORE_DATA_UINT16 => {
            match s.parse::<u16>() {
                Ok(v) => {
                    return Ok(encode_u16(v));
                }
                Err(err) => {
                    return Err(Error::new(ErrorKind::InvalidData, err))
                }
            }
        }
        CORE_DATA_UINT32 => {
            match s.parse::<u32>() {
                Ok(v) => {
                    return Ok(encode_u32(v));
                }
                Err(err) => {
                    return Err(Error::new(ErrorKind::InvalidData, err))
                }
            }
        }
        CORE_DATA_UINT64 => {
            match s.parse::<u64>() {
                Ok(v) => {
                    return Ok(encode_u64(v));
                }
                Err(err) => {
                    return Err(Error::new(ErrorKind::InvalidData, err))
                }
            }
        }
        CORE_DATA_FLOAT32 => {
            match s.parse::<f32>() {
                Ok(v) => {
                    return Ok(encode_f32(v));
                }
                Err(err) => {
                    return Err(Error::new(ErrorKind::InvalidData, err))
                }
            }
        }
        CORE_DATA_FLOAT64 => {
            match s.parse::<f64>() {
                Ok(v) => {
                    return Ok(encode_f64(v));
                }
                Err(err) => {
                    return Err(Error::new(ErrorKind::InvalidData, err))
                }
            }
        }
        CORE_DATA_STRING => {
            return Ok(encode_string(String::from(s)));
        }
        CORE_DATA_BYTES => {
            match hex::decode(s) {
                Ok(bs) => {
                    return Ok(encode_bytes(&bs));
                }
                _ => {
                    return Err(Error::new(ErrorKind::InvalidData, "error bytes"));
                }
            }
        }
        CORE_DATA_LIST => {
            match hex::decode(s) {
                Ok(bs) => {
                    return Ok(encode_data(CORE_DATA_LIST, &bs));
                }
                _ => {
                    return Err(Error::new(ErrorKind::InvalidData, "error data list"));
                }
            }
        }
        CORE_DATA_MAP => {
            match hex::decode(s) {
                Ok(bs) => {
                    return Ok(encode_data(CORE_DATA_MAP, &bs));
                }
                _ => {
                    return Err(Error::new(ErrorKind::InvalidData, "error data map"));
                }
            }
        }
        _ => {
            return Err(Error::new(ErrorKind::InvalidData, "unknown data type"));
        }
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn decode<T: ProtoMessage + std::default::Default>(data: &Vec<u8>) -> Result<Option<T>, Error> {
    if data.len() == 0 {
        return Err(Error::new(ErrorKind::InvalidData, "empty"));
    }
    let mut rdata = data.as_slice();
    let meta = rdata.get_u8();
    match meta {
        CORE_DATA => {
            match ProtoMessage::decode(rdata.chunk()) {
                Ok(m) => {
                    return Ok(Some(m));
                }, 
                Err(err) => {
                    return Err(Error::new(ErrorKind::InvalidData, err));
                },
            };
        }
        CORE_DATA_NULL => {
            return Ok(None);
        }
        CORE_DATA_LIST => {
            match ProtoMessage::decode(rdata.chunk()) {
                Ok(m) => {
                    return Ok(Some(m));
                }, 
                Err(err) => {
                    return Err(Error::new(ErrorKind::InvalidData, err));
                },
            };
        }
        CORE_DATA_MAP => {
            match ProtoMessage::decode(rdata.chunk()) {
                Ok(m) => {
                    return Ok(Some(m));
                }, 
                Err(err) => {
                    return Err(Error::new(ErrorKind::InvalidData, err));
                },
            };
        }
        _ => {
            return Err(Error::new(ErrorKind::InvalidData, "unknown data type"));
        }
    }
}

fn decode_i32(data: &pb::Data) -> Option<i32> {
    if data.bytes.len() > 0 {
        let mut idata = data.bytes.as_slice();
        let meta = idata.get_u8();
        match meta {
            CORE_DATA_INT32 => {
                return Some(idata.get_i32_le());
            },
            _ => {
                return None;
            }
        }
    }
    return None;
}

fn decode_i64(data: &pb::Data) -> Option<i64> {
    if data.bytes.len() > 0 {
        let mut idata = data.bytes.as_slice();
        let meta = idata.get_u8();
        match meta {
            CORE_DATA_INT64 => {
                return Some(idata.get_i64_le());
            },
            _ => {
                return None;
            }
        }
    }
    return None;
}

fn decode_string(data: &pb::Data) -> Option<String> {
    if data.bytes.len() > 0 {
        let mut idata = data.bytes.as_slice();
        let meta = idata.get_u8();
        match meta {
            CORE_DATA_STRING => {
                match from_utf8(idata.chunk()) {
                    Ok(s) => {
                        return Some(s.to_string());
                    }
                    _ => {
                        return None;
                    }
                }
            },
            _ => {
                return None;
            }
        }
    }
    return None;
}

fn encode_boolean(b: bool) -> Vec<u8> {
    let mut buf = vec![];
    buf.put_u8(CORE_DATA_BOOLEAN);
    buf.put_u8(u8::from(b));
    return buf;
}

fn encode_u8(v: u8) -> Vec<u8> {
    let mut buf = vec![];
    buf.put_u8(CORE_DATA_UINT8);
    buf.put_u8(v);
    return buf;
}

fn encode_i8(v: i8) -> Vec<u8> {
    let mut buf = vec![];
    buf.put_u8(CORE_DATA_INT8);
    buf.put_i8(v);
    return buf;
}

fn encode_u16(v: u16) -> Vec<u8> {
    let mut buf = vec![];
    buf.put_u8(CORE_DATA_UINT16);
    buf.put_u16_le(v);
    return buf;
}

fn encode_i16(v: i16) -> Vec<u8> {
    let mut buf = vec![];
    buf.put_u8(CORE_DATA_INT16);
    buf.put_i16_le(v);
    return buf;
}

fn encode_i32(v: i32) -> Vec<u8> {
    let mut buf = vec![];
    buf.put_u8(CORE_DATA_INT32);
    buf.put_i32_le(v);
    return buf;
}

fn encode_u32(v: u32) -> Vec<u8> {
    let mut buf = vec![];
    buf.put_u8(CORE_DATA_UINT32);
    buf.put_u32_le(v);
    return buf;
}

fn encode_i64(v: i64) -> Vec<u8> {
    let mut buf = vec![];
    buf.put_u8(CORE_DATA_INT64);
    buf.put_i64_le(v);
    return buf;
}

fn encode_u64(v: u64) -> Vec<u8> {
    let mut buf = vec![];
    buf.put_u8(CORE_DATA_UINT64);
    buf.put_u64_le(v);
    return buf;
}

fn encode_f32(v: f32) -> Vec<u8> {
    let mut buf = vec![];
    buf.put_u8(CORE_DATA_FLOAT32);
    buf.put_f32_le(v);
    return buf;
}

fn encode_f64(v: f64) -> Vec<u8> {
    let mut buf = vec![];
    buf.put_u8(CORE_DATA_FLOAT64);
    buf.put_f64_le(v);
    return buf;
}

fn encode_string(v: String) -> Vec<u8> {
    let mut buf = vec![];
    buf.put_u8(CORE_DATA_STRING);
    buf.put_slice(v.as_bytes());
    return buf;
}

fn encode_bytes(v: &[u8]) -> Vec<u8> {
    let mut buf = vec![];
    buf.put_u8(CORE_DATA_BYTES);
    buf.put_slice(v);
    return buf;
}

fn encode_data(t:u8, v: &[u8]) -> Vec<u8> {
    let mut buf = vec![];
    buf.put_u8(t);
    buf.put_slice(v);
    return buf;
}

fn encode_key(t: u16, bytes: Vec<u8>) -> Vec<u8> {
    let mut buf = vec![];
    buf.put_u16_le(t);
    buf.put_slice(&bytes);
    return buf;
}

fn encode<T: ProtoMessage + std::default::Default>(t: u8, m: &T) -> Result<Vec<u8>, Error> {
    let mut buf = vec![t];
    let result = ProtoMessage::encode(m, &mut buf);
    let m = match result {
        Ok(()) => buf.as_slice(),
        Err(err) => {
            return Err(Error::new(ErrorKind::InvalidInput, err));
        },
    };
    return Ok(m.to_vec());
}

fn _get_string(m: &pb::DataMap, k: &str) -> Result<Option<String>, Error> {
    if !m.map.contains_key(k) {
        return Ok(None);   
    }
    if let Some(v) = m.map.get(k) {
        match decode_string(&v) {
            Some(s) => {
                return Ok(Some(s));
            }
            _ => {
                return Err(Error::new(ErrorKind::InvalidData, "string"));
            }
        }
    }
    return Ok(None);
}

fn _get_string_required(m: &pb::DataMap, k: &str) -> Result<String, Error> {
    let r = _get_string(m, k)?;
    match r {
        Some(s) => {
            return Ok(s);
        }
        None => {
            return Err(Error::from(ErrorKind::NotFound));
        }
    }
}

fn _get_map(m: &pb::DataMap, k: &str) -> Result<Option<pb::DataMap>, Error> {
    if !m.map.contains_key(k) {
        return Ok(None);
    }
    if let Some(v) = m.map.get(k) {
        match decode::<pb::DataMap>(&v.bytes) {
            Ok(o) => {
                return Ok(o);
            }
            _ => {
                return Err(Error::new(ErrorKind::InvalidData, "data map"));
            }
        }
    }
    return Ok(None);
}

fn _get_map_required(m: &pb::DataMap, k: &str) -> Result<pb::DataMap, Error> {
    let r = _get_map(m, k)?;
    match r {
        Some(sm) => {
            return Ok(sm);
        }
        None => {
            return Err(Error::from(ErrorKind::NotFound));
        }
    }
}

fn hash256(bytes: &Vec<u8>) -> Vec<u8> {
    let mut sha256 = Sha256::new();
    sha256.update(bytes);
    return sha256.finalize().to_vec();
}

fn keccak256(bytes: &Vec<u8>) -> Vec<u8> {
    let hash = &mut [0; 32];
    let mut keccak256 = Keccak::v256();
	keccak256.update(bytes);
	keccak256.finalize(hash);
    return hash.to_vec();
}
    