use crate::ast::expressions::Transform;

use super::Data;

pub fn apply_transform(data: Data, transform: &Transform) -> Data {
    match transform.name() {
        "lowercase" => lowercase(data),
        _ => panic!("Transform '{:?}' does not exist.", transform),
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
        _ => panic!("Lowercase did not like input...")
    }
}