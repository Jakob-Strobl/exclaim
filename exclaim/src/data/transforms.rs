use crate::ast::transforms::Transform;

use super::Data;

pub fn apply_transform(data: Data, transform: &Transform, arguments: Vec<Data>) -> Data {
    // match transform signature: (name, num_arguments)
    match transform.signature() {
        ("chars", 0) => chars(data),
        ("enumerate", 0) => enumerate(data),
        ("lowercase", 0) => lowercase(data),
        ("uppercase", 0) => uppercase(data),
        ("at", 1) => at(data, arguments.get(0).unwrap()),
        ("get", 1) => get(data, arguments.get(0).unwrap()),
        ("take", 1) => take(data, arguments.get(0).unwrap()),
        _ => panic!("Transform '{:?}' does not exist.", transform),
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
        Data::Int(_) => panic!("Cannot transform raw Int to lowercase"),
        Data::Uint(_) => panic!("Cannot transform raw Uint to lowercase"),
        Data::Float(_) => panic!("Cannot transform raw Float to lowercase"),
        Data::Tuple(_) => panic!("Cannot transform raw Tuple to lowercase"),
        Data::Object(_) => panic!("Cannot transform raw Object to lowercase"),
        Data::Array(_) => panic!("Cannot transform raw Array to lowercase"),
    }
}

fn uppercase(data: Data) -> Data {
    match data {
        Data::String(string) => Data::String(string.to_uppercase()),
        Data::Int(_) => panic!("Cannot transform raw Int to uppercase"),
        Data::Uint(_) => panic!("Cannot transform raw Uint to uppercase"),
        Data::Float(_) => panic!("Cannot transform raw Float to uppercase"),
        Data::Tuple(_) => panic!("Cannot transform raw Tuple to uppercase"),
        Data::Object(_) => panic!("Cannot transform raw Object to uppercase"),
        Data::Array(_) => panic!("Cannot transform raw Array to uppercase"),
    }
}

fn at(data: Data, index: &Data) -> Data {
    let index = match index {
        Data::Uint(num) => *num,
        _ => panic!("at only takes a unsigned integer as an argument: {:?}.", index)
    };

    match data {    
        Data::String(string) => {
            if index >= string.len() {
                panic!("at index is out of bounds: {}, length = {}", index, string.len())
            }

            Data::String(string.chars().nth(index).unwrap().to_string())
        },
        Data::Tuple(tuple) => {
            if index >= tuple.len() {
                println!("tuple: {:?}", tuple);
                panic!("at index is out of bounds: {}, length = {}", index, tuple.len())
            }

            tuple[index].clone()
        }
        _ => panic!("at does not transform the given data: {:?}", data),
    }
}

fn get(data: Data, key: &Data) -> Data {
    let key = match key {
        Data::String(string) => string,
        _ => panic!("at only takes a unsigned integer as an argument: {:?}.", key)
    };

    match data {
        Data::Object(object) => {
            match object.get(key) {
                Some(value) => value.clone(),
                None => panic!("The key-value pair does not exist: {:?}", key),
            }
        },
        _ => panic!("get does not transform the given data: {:?}", data)
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