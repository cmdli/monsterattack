// Chris de la Iglesia

extern crate rand;
//extern crate serde;
//extern crate serde_json;

use serde::Serialize;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);
    //let search_string = &args[1];
    let mut file = File::open(&args[1])?;
    let mut file = BufReader::new(file);
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("Content: {}", content);
    let tokens = monsterattack::tokenize(&content);
    println!("Tokens: {:?}", tokens);
    monsterattack::parse_json(&content).expect("Failed to parse");
    Ok(())
}
