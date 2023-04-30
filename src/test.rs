fn _register(mut last: pb::DataMap, address:&String) -> pb::DataMap {
    let mut mm = last.map;
    mm.insert(address.to_string(), pb::Data{
        bytes: encode_string(address),
    });
    last.map = mm;
    return last;
}

fn _post(last: pb::DataList, title:String, content:String) -> pb::DataList {
    let mut mm = HashMap::<String, pb::Data>::new();
    mm.insert("title".to_string(), pb::Data{
        bytes: encode_string(&title),
    });
    mm.insert("content".to_string(), pb::Data{
        bytes: encode_string(&content),
    });
    let nm = pb::DataMap{
        map: mm,
    };
    let nm_bytes = encode(CORE_DATA_MAP, &nm).unwrap();

    let mut list = last.list;
    list.push(pb::Data{
        bytes: nm_bytes,
    });
    return pb::DataList{
        list: list,
    };
}

fn _test_process(m: pb::DataMap, a:i32, b:i32) -> pb::DataMap {
    let list_result = m.map.get("list").unwrap();
    let mut list:pb::DataList = decode(&list_result.bytes).unwrap().unwrap();

    let mut sum_i64:i64 = (a + b).into();
    for data in &list.list {
        let data_i64 = decode_i64(&data).unwrap();
        sum_i64 += data_i64;
    }

    let sum = encode_i64(sum_i64);
    list.list.push(pb::Data{bytes:sum});
    let new_list = encode(CORE_DATA_LIST, &list).unwrap();

    let mut mm = HashMap::<String, pb::Data>::new();
    mm.insert("list".to_string(), pb::Data{
        bytes: new_list,
    });
    let nm = pb::DataMap{
        map: mm,
    };
    return nm;
}

fn _test_list(l0: pb::DataList, a:i32, b:i32) -> pb::DataList {
    let mut sum_i64:i64 = (a + b).into();
    for d in l0.list {
        if let Ok(Some(m)) = decode::<pb::DataMap>(&d.bytes) {
            if let Some(list_result) = m.map.get("list") {
                if let Ok(Some(list)) = decode::<pb::DataList>(&list_result.bytes) {
                    for data in &list.list {
                        if let Some(data_i64) = decode_i64(&data) {
                            sum_i64 += data_i64;
                        }
                    }
                }
            }
        }
    }

    let sum = encode_i64(sum_i64);
    let list = vec![
        pb::Data{bytes:sum},
    ];
    let new_list = encode(CORE_DATA_LIST, &pb::DataList{
        list: list,
    }).unwrap();
    let mut mm = HashMap::<String, pb::Data>::new();
    mm.insert("list".to_string(), pb::Data{
        bytes: new_list,
    });
    let nm = pb::DataMap{
        map: mm,
    };
    let nm_bytes = encode(CORE_DATA_MAP, &nm).unwrap();
    return pb::DataList{
        list: vec![pb::Data{
            bytes: nm_bytes,
        }],
    };
}