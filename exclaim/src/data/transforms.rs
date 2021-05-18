use crate::ast::transforms::Transform;

use super::Data;

pub fn apply_transform(data: Data, transform: &Transform, arguments: Vec<Data>) -> Data {
    // match transform signature: (name, num_arguments)
    match transform.signature() {
        ("array", 0) => array(data),
        ("chars", 0) => chars(data),
        ("enumerate", 0) => enumerate(data),
        ("lowercase", 0) => lowercase(data),
        ("object", 0) => object(data),
        ("string", 0) => string(data),
        ("tuple", 0) => tuple(data),
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
        Data::Tuple(_) => panic!("Unimplemented"),
        Data::Object(_) => panic!("Unimplemented"),
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

fn lowercase(data: Data) -> Data {
    match data {
        Data::String(string) => Data::String(string.to_lowercase()),
        _ => panic!("Cannot transform input to lowercase"),
    }
}

fn object(data: Data) -> Data {
    match data {
        Data::String(_) | Data::Int(_) | Data::Uint(_) | Data::Float(_) => panic!("Unable to call `array` on scalar types."),
        Data::Tuple(_) => panic!("Unimplemented"),
        Data::Object(_) => data,
        Data::Array(_) => panic!("Unimplemented"),
        Data::Option(_) => panic!("Unable to call `object` on wrapper types.")
    }
}

fn string(data: Data) -> Data {
    match data {
        Data::Uint(number) => {
            Data::String(number.to_string())
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
        Data::Object(_) => panic!("Unimplemented"),
        Data::Array(_) => panic!("Unimplemented"),
        Data::Option(_) => panic!("Unable to call `tuple` on wrapper types.")
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
                _ => panic!("at does not transform the given data: {:?}", data),
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