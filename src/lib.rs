// Chris de la Iglesia, 2019

use std::collections::HashMap;
use std::vec::Vec;
use std::result::Result;
use serde::Deserialize;

pub struct Attack {
    name: String,
    to_hit: i64,
//    damage: String,
    base_damage: i64,
    num_d4: i64,
    num_d6: i64,
    num_d8: i64,
    num_d10: i64,
    num_d12: i64,
    num_d20: i64,
}

#[derive(serde::Deserialize)]
pub struct StatBlock {
    max_hp: i64,
    to_hit: i64,
    armor: i64,
    attacks: Vec<Attack>,
}

pub fn tokenize(input: &str) -> Result<Vec<String>,&str> {
    let mut text = input;
    text = text.trim();
    let mut output = Vec::new();
    while text.len() > 0 {
        let next = &(text[0..1]);
        if "{}\",:".contains(next) {
            output.push(String::from(next));
            text = &text[1..];
            continue;
        }
        if let Some(target_i) = text.find(|c: char| "{}\",:".contains(c)) {
            output.push(String::from(&(text[0..target_i])));
            text = &text[target_i..];
        } else {
            return Err("Error tokenizing");
        }
    }
    return Ok(output);
}

pub fn parse_json(input: &str) -> Result<HashMap<String,String>,String> {
    let tokens = tokenize(input)?;
    let mut i = 0;
    parse_object(&tokens, &mut i)?;
    return Ok(HashMap::new());
}

fn expect(input: &Vec<String>, i: &mut usize, value: &str) -> Result<String,String> {
    if *i > input.len() {
        return Err("Not enough tokens".to_string());
    }
    let target = &input[*i];
    if value == target {
        *i += 1;
        Ok(target.to_string())
    } else {
        Err("Expected value not there".to_string())
    }
}

fn parse_object(input: &Vec<String>, i: &mut usize) -> Result<HashMap<String,String>,String> {
    expect(input, i, "{")?;
    let key = parse_key(input, i)?;
    expect(input, i, "}")?;
    Ok(HashMap::new())
}

fn parse_key<'a>(input: &'a Vec<String>, i: &mut usize) -> Result<String,String> {
    expect(input, i, "\"");
    let key = input[*i].to_string();
    *i += 1;
    expect(input, i, "\"");
    Ok(key)
}

