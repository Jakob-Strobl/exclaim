use std::collections::BTreeMap;

use crate::ast::transforms::Transform;

use super::Data;

pub fn apply_transform(data: Data, transform: &Transform, arguments: Vec<Data>) -> Data {
    // match transform signature: (name, num_arguments)
    match transform.signature() {
        ("array", 0) => array(data),
        ("chars", 0) => chars(data),
        ("enumerate", 0) => enumerate(data),
        ("float", 0) => float(data),
        ("int", 0) => int(data),
        ("lowercase", 0) => lowercase(data),
        ("object", 0) => object(data),
        ("string", 0) => string(data),
        ("tuple", 0) => tuple(data),
        ("uint", 0) => uint(data),
        ("unwrap", 0) => unwrap(data),
        ("uppercase", 0) => uppercase(data),
        ("get", 1) => get(data, arguments.get(0).unwrap()),
        ("take", 1) => take(data, arguments.get(0).unwrap()),
        _ => panic!("Transform '{:?}' does not exist.", transform),
    }
}

fn array(data: Data) -> Data {
    match data {
        Data::String(_) | Data::Int(_) | Data::Uint(_) | Data::Float(_) => panic!("Unable to call `array` on scalar types."),
        Data::Tuple(tuple) => {
            Data::Array(tuple.to_vec())
        },
        Data::Object(object) => {
            let mut array = vec![];
            for (key, value) in object.into_iter() {
                let key = Data::String(key);
                let pair = Data::Tuple(Box::new([key, value]));
                array.push(pair);
            }

            Data::Array(array)
        },
        Data::Array(_) => data,
        Data::Option(_) => panic!("Unable to call `array` on wrapper types.")
    }
}

fn chars(data: Data) -> Data {
    match data {
        Data::String(string) => Data::Array(string.chars().map(|c| Data::String(c.to_string())).collect()),
        _ => panic!("chars expects string as input")
    }
}

fn enumerate(data: Data) -> Data {
    match data {
        Data::Array(array) => {
            let mut enumerated_array = vec![];
            let mut index = 0; 
            
            for data in array {
                enumerated_array.push(Data::Tuple(Box::new([data, Data::Uint(index)])));
                index += 1;
            }

            Data::Array(enumerated_array)
        },
        _ => panic!("enumerate expects an array as input.")
    }
}

fn float(data: Data) -> Data {
    match data {
        Data::String(string) => {
            let number: f64 = string.parse().unwrap();
            Data::Float(number)
        }
        _ => panic!("unimplemented"),
    }
}

fn int(data: Data) -> Data {
    match data {
        Data::String(string) => {
            let number: isize = string.parse().unwrap();
            Data::Int(number)
        }
        _ => panic!("unimplemented"),
    }
}

fn lowercase(data: Data) -> Data {
    match data {
        Data::String(string) => Data::String(string.to_lowercase()),
        _ => panic!("Cannot transform input to lowercase"),
    }
}

fn object(data: Data) -> Data {
    match data {
        Data::String(_) | Data::Int(_) | Data::Uint(_) | Data::Float(_) => panic!("Unable to call `array` on scalar types."),
        Data::Tuple(tuple) => {
            let mut object = BTreeMap::new();
            for (index, item) in tuple.iter().enumerate() {
                object.insert(index.to_string(), item.clone());
            }

            Data::Object(object)
        },
        Data::Object(_) => data,
        Data::Array(array) => {
            let mut object = BTreeMap::new();
            for (index, item) in array.iter().enumerate() {
                object.insert(index.to_string(), item.clone());
            }

            Data::Object(object)
        },
        Data::Option(_) => panic!("Unable to call `object` on wrapper types.")
    }
}

fn string(data: Data) -> Data {
    match data {
        Data::Uint(uint) => {
            Data::String(uint.to_string())
        }
        Data::Int(int) => {
            Data::String(int.to_string())
        }
        Data::Float(float) => {
            Data::String(float.to_string())
        }
        Data::Tuple(_) | Data::Object(_) | Data::Array(_) => panic!("Unable to call `string` on compound types."),
        Data::Option(_) => panic!("Unable to call `string` on wrapper types."),
        _ => panic!("Invalid input type for `string`.")
    }
}

fn tuple(data: Data) -> Data {
    match data {
        Data::String(_) | Data::Int(_) | Data::Uint(_) | Data::Float(_) => panic!("Unable to call `array` on scalar types."),
        Data::Tuple(_) => data,
        Data::Object(object) => {
            let mut keys = vec![];
            let mut values = vec![];

            for (key, value) in object.iter() {
                keys.push(Data::String(key.to_string()));
                values.push(value.clone());
            }

            let keys = Data::Array(keys);
            let values = Data::Array(values);

            Data::Tuple(Box::new([keys, values]))
        },
        Data::Array(array) => {
            Data::Tuple(array.into_boxed_slice())
        },
        Data::Option(_) => panic!("Unable to call `tuple` on wrapper types.")
    }
}

fn uint(data: Data) -> Data {
    match data {
        Data::String(string) => {
            let number: usize = string.parse().unwrap();
            Data::Uint(number)
        }
        _ => panic!("unimplemented"),
    }
}

fn unwrap(data: Data) -> Data {
    match data {
        Data::Option(option) => {
            match option {
                Some(value) => *value, // Deref the Box<T>
                None => panic!("Tried to unwrap nothing!"),
            }
        }
        _ => panic!("unwrap can only transform Options."),
    }
}

fn uppercase(data: Data) -> Data {
    match data {
        Data::String(string) => Data::String(string.to_uppercase()),
        _ => panic!("Cannot transform input to uppercase"),
    }
}

fn get(data: Data, key: &Data) -> Data {
    match key {
        Data::String(key) => {
            match data {
                Data::Object(object) => {
                    match object.get(key) {
                        Some(value) => Data::Option(Some(Box::new(value.clone()))),
                        None => Data::Option(None),
                    }
                },
                _ => panic!("get does not transform the given data: {:?}", data)
            }
        },
        Data::Uint(index) => {
            match data {
                Data::Array(array) => {
                    if *index >= array.len() {
                        return Data::Option(None)
                    }

                    Data::Option(Some(Box::new(array[*index].clone())))
                }    
                Data::Tuple(tuple) => {
                    if *index >= tuple.len() {
                        return Data::Option(None)
                    }
        
                    Data::Option(Some(Box::new(tuple[*index].clone())))
                }
                _ => panic!("get does not transform the given data: {:?}", data),
            }
        }
        _ => panic!("get only takes a string as an argument: {:?}.", key)
    }
}

fn take(data: Data, uint: &Data) -> Data {
    let take = match uint {
        Data::Uint(num) => *num,
        _ => panic!("at only takes a unsigned integer as an argument: {:?}.", uint)
    };

    match data {
        Data::Array(array) => {
            let take_slice = array.split_at(take).0;
            let mut new_array = Vec::with_capacity(take_slice.len());
            new_array.clone_from(&take_slice.to_vec());
            Data::Array(new_array)
        },
        _ => panic!("take does not transform the given data: {:?}", data)
    }
}