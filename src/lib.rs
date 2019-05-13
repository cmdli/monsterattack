// Chris de la Iglesia, 2019

extern crate serde;
extern crate rand;

use rand::prelude::*;
use rand::{thread_rng, Rng};
use std::vec::Vec;
use std::result::Result;
use serde::Deserialize;

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
    pub fn from_str(string: &str) -> Result<StatBlock,serde_json::error::Error> {
        return serde_json::from_str(string);
    }
}

fn roll_dice(size: i64, num: i64, rng: &mut ThreadRng) -> i64 {
    let mut total = 0;
    for i in (0..num) {
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

pub fn fight(creature1: &StatBlock, creature2: &StatBlock) -> Option<String> {
    let mut rng = rand::thread_rng();
    let attack1 = match creature1.attacks.first() {
        Some(x) => {
            x
        }
        None => {
            return None;
        }
    };
    let attack2 = match creature2.attacks.first() {
        Some(x) => x,
        None => {return None;}
    };
    let mut creature1dmg = 0;
    let mut creature2dmg = 0;
    loop {
        if creature1dmg >= creature1.max_hp {
            println!("Creature 2 won!");
            break;
        }
        if hits(attack1, creature2, &mut rng) {
            let dmg = damage(attack1, &mut rng);
            println!("{} (1) does {} damage from {}", creature1.name, dmg, attack1.name);
            creature2dmg += dmg;
        } else {
            println!("{} (1) misses with {}", creature1.name, attack1.name);
        }
        
        if creature2dmg >= creature2.max_hp {
            println!("Creature 1 won!");
            break;
        }
        if hits(attack2, creature2, &mut rng) {
            let dmg = damage(attack1, &mut rng);
            println!("{} (2) does {} damage from {}", creature2.name, dmg, attack2.name);
            creature1dmg += dmg;
        } else {
            println!("{} (2) misses with {}", creature2.name, attack2.name);
        }
    }
    Some(String::from("Done"))
}

