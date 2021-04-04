use std::{
    fs::File, 
    io::Read
};

use criterion::Bencher;
use exclaim;

fn read_file_to_string(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Benchmark: Tests lexer speed when processing a file of just one long string literal 
/// The output should just be one string literal 
pub fn bench_string_literal(b: &mut Bencher) {
    let contents = read_file_to_string("data/long_string_literal.txt").unwrap();
    b.iter(|| exclaim::run_lexer(&contents));
}