fn _register(last: pb::DataMap, name:&String, address:&String) -> pb::DataMap {
    let mut mm = HashMap::<String, pb::Data>::new();
    mm.insert("index".to_string(), pb::Data{
        bytes: encode_i64(last.map.len() as i64),
    });
    mm.insert("name".to_string(), pb::Data{
        bytes: encode_string(name),
    });
    let nm = pb::DataMap{
        map: mm,
    };
    return map_put(last, address.to_string(), CORE_DATA_MAP, &nm)
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
    return list_push(last, CORE_DATA_MAP, &nm);
}

fn _apply(last: pb::DataList, index:i64, info_bytes:Vec<u8>) -> pb::DataList {
    if let Ok(Some(mut nm)) = get_map_from_list(&last, index as usize) {
        if let Ok(Some(mut ll)) = get_list_from_map(&nm, "list") {
            let mut list = ll.list;
            list.push(pb::Data{
                bytes: info_bytes,
            });
            ll.list = list;
            nm = map_put(nm, "list".to_string(), CORE_DATA_LIST, &ll);
        }else{
            let ll = pb::DataList{
                list: vec![
                    pb::Data{
                        bytes: info_bytes,
                    },
                ],
            };
            nm = map_put(nm, "list".to_string(), CORE_DATA_LIST, &ll);
        }
        return list_update(last, index as usize, CORE_DATA_MAP, &nm);
    }
    return last;
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