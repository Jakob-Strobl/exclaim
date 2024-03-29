use std::collections::BTreeMap;

use crate::{ast::transforms::Transform, data::traits::Renderable};

use super::Data;

pub fn apply_transform(data: Data, transform: &Transform, arguments: Vec<Data>) -> Data {
    // match transform signature: (name, num_arguments)
    match transform.name() {
        "array" => array(data),
        "chars" => chars(data),
        "concat" => {
            match transform.num_arguments() {
                0 => concat(data),
                1 => concat_scalar(data, arguments.get(0).unwrap()),
                _ => panic!("Wrong number of arguments for concat"),
            }
        }
        "enumerate" => enumerate(data),
        "float" => float(data),
        "get" => {
            match transform.num_arguments() {
                1 => get(data, arguments.get(0).unwrap()),
                _ => panic!("Wrong number of arguments for get"),
            }
        },
        "int" => int(data),
        "len" => len(data),
        "lowercase" => lowercase(data),
        "object" => object(data),
        "string" => string(data),
        "take" => {
            match transform.num_arguments() {
                1 => take(data, arguments.get(0).unwrap()),
                2 => take_lower_upper(data, arguments.get(0).unwrap(), arguments.get(1).unwrap()),
                _ => panic!("Wrong number of arguments for take"),
            }
        },
        "tuple" => tuple(data),
        "uint" => uint(data),
        "unwrap" => unwrap(data),
        "uppercase" => uppercase(data),

        // Reserved transformation names
        "map" | "filter" | "reduce" => panic!("Transformation is reserved."),
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

fn concat(data: Data) -> Data {
    match data {
        Data::Array(array) => {
            let mut concatenated = String::new();

            for data in array {
                if data.is_scalar() {
                    concatenated.push_str(&data.render())
                } else {
                    panic!("Found non-scalar element while concatenating an array")
                }
            }

            Data::String(concatenated)
        },
        _ => panic!("chars expects string as input")
    }
}

fn concat_scalar(mut data: Data, scalar: &Data) -> Data {
    let scalar = match scalar {
        Data::String(string) => string.to_string(),
        Data::Int(int) => int.to_string(),
        Data::Uint(uint) => uint.to_string(),
        Data::Float(float) => float.to_string(),
        _ => panic!("Concat can only take scalars as an argument"),
    };

    match &mut data {
        Data::String(string) => {
            string.push_str(&scalar);
        },
        _ => panic!("concat expects string as input")
    }

    data
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
        Data::Float(_) => data,
        Data::String(string) => {
            let number: f64 = string.parse().unwrap();
            Data::Float(number)
        }
        Data::Uint(uint) => {
            Data::Float(uint as f64)
        }
        Data::Int(int) => {
            Data::Float(int as f64)
        }
        Data::Array(_) | Data::Tuple(_) | Data::Object(_) => panic!("Unable to call `float` transformation on compound types."),
        Data::Option(_) => panic!("Unable to call `float` transformation on wrapper types."),
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

fn int(data: Data) -> Data {
    match data {
        Data::Int(_) => data,
        Data::String(string) => {
            let number: isize = string.parse().unwrap();
            Data::Int(number)
        }
        Data::Uint(uint) => {
            Data::Int(uint as isize)
        }
        Data::Float(float) => {
            Data::Int(float as isize)
        }
        Data::Array(_) | Data::Tuple(_) | Data::Object(_) => panic!("Unable to call `int` transformation on compound types."),
        Data::Option(_) => panic!("Unable to call `int` transformation on wrapper types."),
    }
}

fn len(data: Data) -> Data {
    let length = match data {
        Data::String(string) => string.len(),
        Data::Int(_) => panic!("Unable to call `len` on Int."),
        Data::Uint(_) => panic!("Unable to call `len` on Uint."),
        Data::Float(_) => panic!("Unable to call `len` on Float."),
        Data::Array(array) => array.len(),
        Data::Tuple(tuple) => tuple.len(),
        Data::Object(_) => panic!("Unable to call `len` on Object."),
        Data::Option(_) => panic!("Unable to call `len` on wrapper types."),
    };

    Data::Uint(length)
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
        Data::String(_) => data,
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

fn take_lower_upper(data: Data, lower: &Data, upper: &Data) -> Data {
    let lower = match lower {
        Data::Uint(num) => *num,
        _ => panic!("take only takes a unsigned integer as an argument: {:?}.", lower)
    };

    let upper = match upper {
        Data::Uint(num) => *num,
        _ => panic!("take only takes a unsigned integer as an argument: {:?}.", upper)
    };

    match data {
        Data::Array(array) => {
            if lower >= array.len() {
                panic!("Lower range is greater than the length of the array: {} >= {}", upper, array.len())
            }
            // Only Greater Than since the upper bound is exclusive
            if upper > array.len() {
                panic!("Upper range is greater than the length of the array: {} >= {}", upper, array.len())
            }

            let sub_array = array[lower..upper].to_vec();

            Data::Array(sub_array)
        },
        _ => panic!("take does not transform the given data: {:?}", data)
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
        Data::Uint(_) => data,
        Data::String(string) => {
            let number: usize = string.parse().unwrap();
            Data::Uint(number)
        }
        Data::Int(int) => {
            if int < 0 {
                panic!("Unable to transform a negative integer into an unsigned integer")
            }
            Data::Uint(int as usize)
        }
        Data::Float(float) => {
            if float < 0.0 {
                panic!("Unable to transform a negative float into an unsigned integer")
            }
            Data::Uint(float as usize)
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