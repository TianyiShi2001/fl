use std::collections::HashMap; // TODO: replace hashmap

#[derive(Debug)]
pub struct Char(pub Vec<String>);

impl Char {
    pub fn with_height(height: usize) -> Self {
        Self(vec![String::with_capacity(10); height])
    }
}

#[derive(Debug)]
pub struct Alphabet(pub HashMap<u8, Char>);

impl Alphabet {
    pub fn new() -> Self {
        Self(HashMap::with_capacity(102))
    }
}
