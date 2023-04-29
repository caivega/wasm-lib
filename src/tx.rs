fn get_public(pub_key: &String) -> Result<(u16, PublicKey), Error> {
    let s = pub_key.strip_prefix("eth.").unwrap_or(&pub_key);
    match hex::decode(s) {
        Ok(ss) => {
            let pk = PublicKey::parse_slice(&ss, Some(PublicKeyFormat::Full)).unwrap();
            return Ok((ETH, pk));
        }
        _ => {
            return Err(Error::new(ErrorKind::InvalidData, "public key"));
        }
    }
}

fn get_secret(secret: &String) -> Result<(u16, SecretKey, PublicKey, String), Error> {
    let s = secret.strip_prefix("eth.").unwrap_or(&secret);
    match hex::decode(s) {
        Ok(ss) => {
            let priv_key = SecretKey::parse_slice(&ss).unwrap();
            let pub_key = PublicKey::from_secret_key(&priv_key);

            let pub_bytes = pub_key.serialize();
            let hash = keccak256(&pub_bytes[1..].to_vec());
            let address_hex = hex::encode(hash[12..].to_vec());
            let address_bytes: &mut [u8] = &mut address_hex.as_bytes().to_owned();
            let address_hash = keccak256(&address_hex.as_bytes().to_vec());
            for i in 0..address_bytes.len() {
                let mut hash_byte = address_hash[i/2];
                if i%2 == 0 {
                    hash_byte = hash_byte >> 4
                } else {
                    hash_byte &= 0xf
                }
                if address_bytes[i] > b'9' && hash_byte > 7 {
                    address_bytes[i] -= 32
                }
            }
            let address = match from_utf8(address_bytes) {
                Ok(a) => a,
                _ => {
                    return Err(Error::new(ErrorKind::InvalidData, "address"));
                }
            };

            Ok((ETH, priv_key, pub_key, String::from("eth.0x".to_owned() + address)))
        }
        _ => {
            return Err(Error::new(ErrorKind::InvalidData, "secret"));
        }
    }
}

fn get_address(s: &String) -> Result<Vec<u8>, Error> {
    let ss = s.strip_prefix("eth.0x").or_else(|| s.strip_prefix("eth.0X")).unwrap_or(&s);
    match hex::decode(ss) {
        Ok(address) => {
            return Ok(encode_key(ETH, address));
        }
        _ => {
            return Err(Error::new(ErrorKind::InvalidData, "address"));
        }
    }
}

fn dump(name: &str, bytes: &Vec<u8>) {
    println!("{}: {:?}", name, hex::encode(bytes));
}

fn _get_accounts(m: &pb::DataMap, k: &str) -> Result<Option<Vec<Vec<u8>>>, Error> {
    match _get_string(m, k)? {
        Some(s) => {
            let inputs:Vec<&str> = s.split(",").collect();
            let mut list:Vec<Vec<u8>> = Vec::new();
            for _input in inputs {
                let address = get_address(&_input.to_string())?;
                list.push(address);
            }
            return Ok(Some(list));
        }
        None => {
            return Ok(None);
        }
    }
}

fn _get_params(m: &pb::DataMap, k: &str) -> Result<Option<Vec<Vec<u8>>>, Error> {
    match _get_string(m, k)? {
        Some(s) => {
            let params:Vec<&str> = s.split(",").collect();
            let mut list:Vec<Vec<u8>> = Vec::new();
            for p in params {
                let ps:Vec<&str> = p.split(":").collect();
                if ps.len() != 2 {
                    return Err(Error::new(ErrorKind::InvalidData, "parameter"));
                }
                let t = get_type(ps[1]);
                if t == 0 {
                    return Err(Error::new(ErrorKind::InvalidData, "parameter type"));
                }
                let data = get_from_string(t, ps[0])?;
                list.push(data);
            }
            return Ok(Some(list));
        }
        None => {
            return Ok(None);
        }
    }
}

fn _get_payload(m: &pb::DataMap, has_content: bool) -> Result<pb::PayloadInfo, Error> {
    let pm = _get_map_required(&m, "payload")?;
    let mut infos: Vec<pb::DataInfo> = Vec::new();
    if let Ok(Some(cm)) = _get_map(&pm, "code") {
        let mut code_info = pb::CodeInfo{
            name: "".to_string(),
            code: vec![],
            abi: vec![],
        };

        if let Ok(Some(name)) = _get_string(&cm, "name") {
            code_info.name = name;
        }
        let data = _get_string_required(&cm, "data")?;
        match hex::decode(data) {
            Ok(data_bytes) => {
                code_info.code = data_bytes;
            }
            _ => {
                return Err(Error::new(ErrorKind::InvalidData, "data in code info"));
            }
        }
        if let Ok(Some(abi)) = _get_string(&cm, "abi") {
            match hex::decode(abi) {
                Ok(abi_bytes) => {
                    code_info.abi = abi_bytes;
                }
                _ => {
                    return Err(Error::new(ErrorKind::InvalidData, "abi in code info"));
                }
            }
        }
        match encode(CORE_CODE_INFO, &code_info) {
            Ok(code_bytes) => {
                let h256 = hash256(&code_bytes);
                let mut info = pb::DataInfo{
                    hash: h256,
                    content: vec![],
                };
                if has_content {
                    info.content = code_bytes;
                }
                infos.push(info);
            }
            _ => {
                return Err(Error::new(ErrorKind::InvalidData, "code info"));
            }
        }
    }
    
    if let Ok(Some(cm)) = _get_map(&pm, "contract") {
        let mut contract_info = pb::ContractInfo{
            method:"".to_string(),
            inputs: vec![],
            outputs: vec![],
            params: vec![],
        };

        contract_info.method = _get_string_required(&cm, "method")?;

        if let Some(inputs) = _get_accounts(&cm, "inputs")? {
            contract_info.inputs = inputs;
        }
        if let Some(outputs) = _get_accounts(&cm, "outputs")? {
            contract_info.outputs = outputs;
        }
        if let Some(params) = _get_params(&cm, "params")? {
            contract_info.params = params;
        }

        match encode(CORE_CONTRACT_INFO, &contract_info) {
            Ok(contract_bytes) => {
                let h256 = hash256(&contract_bytes);
                let mut info = pb::DataInfo{
                    hash: h256,
                    content: vec![],
                };
                if has_content {
                    info.content = contract_bytes;
                }
                infos.push(info);
            }
            _ => {
                return Err(Error::new(ErrorKind::InvalidData, "contract info"));
            }
        }
    }

    if let Ok(Some(cm)) = _get_map(&pm, "page") {
        let mut page_info = pb::PageInfo{
            name: "".to_string(),
            data: vec![],
        };

        if let Ok(Some(name)) = _get_string(&cm, "name") {
            page_info.name = name;
        }
        let data = _get_string_required(&cm, "data")?;
        match hex::decode(data) {
            Ok(data_bytes) =>  {
                page_info.data = data_bytes;
            }
            _ => {
                return Err(Error::new(ErrorKind::InvalidData, "data in page info"));
            }
        }
        match encode(CORE_PAGE_INFO, &page_info) {
            Ok(page_bytes) => {
                let h256 = hash256(&page_bytes);
                let mut info = pb::DataInfo{
                    hash: h256,
                    content: vec![],
                };
                if has_content {
                    info.content = page_bytes;
                }
                infos.push(info);
            }
            _ => {
                return Err(Error::new(ErrorKind::InvalidData, "contract info"));
            }
        }
    }
    return Ok(pb::PayloadInfo{
        infos:infos,
    });
}

fn _sign_tx(m: pb::DataMap) -> Result<String, Error> {
    let from = _get_string_required(&m, "from")?;
    let secret = _get_string_required(&m, "secret")?;
    let to = _get_string_required(&m, "to")?;
    let gas = _get_string_required(&m, "gas")?;
    let sequence = _get_string_required(&m, "sequence")?;
    
    // println!("from: {}", from);
    // println!("secret: {}", secret);
    // println!("to: {}", to);
    // println!("gas: {}", gas);
    // println!("sequence: {}", sequence);

    let from_address = get_address(&from)?;
    let to_address = get_address(&to)?;

    let (key_type, priv_key, pub_key, _address) = get_secret(&secret)?;
    // dump("public", &pub_key.serialize().to_vec());
    // dump("private", &priv_key.serialize().to_vec());

    let mut tx = pb::Transaction {
        transaction_type: CORE_TRANSACTION as u32,
        account: from_address,
        sequence: sequence.parse::<u64>().unwrap(),
        gas: gas.parse::<u64>().unwrap(),
        destination: to_address,
        payload: None,
        public_key: encode_key(key_type, pub_key.serialize().to_vec()),
        signature: vec![],
    };

    let has_payload = m.map.contains_key("payload");
    if has_payload {
        let payload = _get_payload(&m, false)?;
        tx.payload = Some(payload);
    }

    let buf = encode(CORE_TRANSACTION, &tx).unwrap();
    let tx_bytes = buf.to_vec();

    let h256 = hash256(&tx_bytes);
    
    let msg = Message::parse_slice(&h256).unwrap();
    let (sig, _) = sign(&msg, &priv_key);
    let sig_bytes = sig.serialize();
    tx.signature = sig_bytes.to_vec();
    // dump("signature", &sig_bytes.to_vec());

    if has_payload {
        let payload = _get_payload(&m, true)?;
        tx.payload = Some(payload);
    }

    let buf = encode(CORE_TRANSACTION, &tx).unwrap();
    let tx_bytes = buf.to_vec();

    // let h = hash256(&tx_bytes);
    // println!("hash: {:?} {:?}", hex::encode(h256), hex::encode(h));

    return Ok(hex::encode(tx_bytes));
}