use std::fs::File;
use std::io::prelude::*;
use toml::Value;
use rug::{Assign, Integer};
use rug::rand::RandState;

fn get_config(file_name : &str) -> (i64, i64, i64) {
    let mut file = File.open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let value = contents.parse::<Value>().unwrap();
    let bit_length = value["params"]["bit_length"].as_integer().unwrap();
    let p_length = value["params"]["p_length"].as_integer().unwrap();
    let witness = value["params"]["witness"].as_integer().unwrap();
    (bit_length, p_length, witness)
}





