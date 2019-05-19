// Chris de la Iglesia, 2019

extern crate serde;
extern crate rand;

use rand::prelude::*;
use rand::Rng;
use std::vec::Vec;
use std::result::Result;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Deserialize, Debug)]
pub struct Attack {
    #[serde(default)]
    name: String,
    #[serde(default)]
    to_hit: i64,
    #[serde(default)]
    base_damage: i64,
    #[serde(default)]
    num_d4: i64,
    #[serde(default)]
    num_d6: i64,
    #[serde(default)]
    num_d8: i64,
    #[serde(default)]
    num_d10: i64,
    #[serde(default)]
    num_d12: i64,
    #[serde(default)]
    num_d20: i64,
}

#[derive(Deserialize, Debug)]
pub struct StatBlock {
    #[serde(default)]
    name: String,
    #[serde(default)]
    max_hp: i64,
    #[serde(default)]
    armor: i64,
    #[serde(default)]
    attacks: Vec<Attack>,
}

impl StatBlock {
    pub fn from_str(string: &str) -> Result<StatBlock,&'static str> {
        match serde_json::from_str(string) {
            Ok(x) => Ok(x),
            Err(_) => Err("Could not parse json"),
        }
    }
    
    pub fn from_file(filename: &str) -> Result<StatBlock,&'static str> {
        let mut content = String::new();
        let file = match File::open(filename) {
            Ok(x) => x,
            Err(_) => {
                return Err("Could not open file");
            }
        };
        let mut file_reader = BufReader::new(file);
        match file_reader.read_to_string(&mut content) {
            Err(_) => {
                return Err("Could not read file contents");
            }
            Ok(x) => x,
        };
        match StatBlock::from_str(&content) {
            Ok(x) => Ok(x),
            Err(x) => Err(x),
        }
    }
}

fn roll_dice(size: i64, num: i64, rng: &mut ThreadRng) -> i64 {
    let mut total = 0;
    for _ in 0..num {
        total += rng.gen_range(1,size);
    }
    total
}

fn hits(attack: &Attack, creature: &StatBlock, rng: &mut ThreadRng) -> bool {
    rng.gen_range(1,21) + attack.to_hit >= creature.armor
}

fn damage(attack: &Attack, rng: &mut ThreadRng) -> i64 {
    // TODO: add in dice rolls
    let mut dmg = attack.base_damage;
    dmg += roll_dice(4, attack.num_d4, rng);
    dmg += roll_dice(6, attack.num_d6, rng);
    dmg += roll_dice(8, attack.num_d8, rng);
    dmg += roll_dice(10, attack.num_d10, rng);
    dmg += roll_dice(12, attack.num_d12, rng);
    dmg += roll_dice(20, attack.num_d20, rng);
    dmg
}

fn attack(attacker: &StatBlock, defender: &StatBlock, rng: &mut ThreadRng) -> i64 {
    let attack = attacker.attacks.choose(rng).unwrap();
    if hits(attack, defender, rng) {
        let dmg = damage(attack, rng);
        println!("{} does {} damage from {}", attacker.name, dmg, attack.name);
        dmg
    } else {
        println!("{} misses with {}", attacker.name, attack.name);
        0
    }
}

pub fn fight(creature1: &StatBlock, creature2: &StatBlock) -> Option<String> {
    let mut thread_rng = rand::thread_rng();
    let rng = &mut thread_rng;
    let mut creature1dmg = 0;
    let mut creature2dmg = 0;
    loop {
        if creature1dmg >= creature1.max_hp {
            println!("{} won!", creature2.name);
            break;
        }
        creature2dmg += attack(creature1, creature2, rng);
        if creature2dmg >= creature2.max_hp {
            println!("{} won!", creature1.name);
            break;
        }
        creature1dmg += attack(creature2, creature1, rng);
    }
    Some(String::from("Done"))
}

