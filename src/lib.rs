// Chris de la Iglesia, 2019

extern crate rand;
extern crate serde;

use rand::prelude::*;
use rand::Rng;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::result::Result;
use std::vec::Vec;

#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
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

#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct StatBlock {
    #[serde(default)]
    name: String,
    #[serde(default)]
    max_hp: i64,
    #[serde(default)]
    armor: i64,
    #[serde(default)]
    num_attacks: i64,
    #[serde(default)]
    attacks: Vec<Attack>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Creature<'a> {
    id: i64,
    hp: i64,
    team: i64,
    stats: &'a StatBlock,
}

impl StatBlock {
    pub fn from_str(string: &str) -> Result<StatBlock, &'static str> {
        match serde_json::from_str(string) {
            Ok(x) => Ok(x),
            Err(_) => Err("Could not parse json"),
        }
    }

    pub fn from_file(filename: &str) -> Result<StatBlock, &'static str> {
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
        StatBlock::from_str(&content)
    }
}

impl<'a> Creature<'a> {
    pub fn new(stats: &'a StatBlock, team: i64, id: i64) -> Creature<'a> {
        Creature {
            id: id,
            hp: stats.max_hp,
            team: team,
            stats: stats,
        }
    }
}

fn roll_dice(size: i64, num: i64, rng: &mut ThreadRng) -> i64 {
    let mut total = 0;
    for _ in 0..num {
        total += rng.gen_range(1, size);
    }
    total
}

fn hits(attack: &Attack, creature: &StatBlock, rng: &mut ThreadRng) -> bool {
    rng.gen_range(1, 21) + attack.to_hit >= creature.armor
}

fn damage(attack: &Attack, rng: &mut ThreadRng) -> i64 {
    let mut dmg = attack.base_damage;
    dmg += roll_dice(4, attack.num_d4, rng);
    dmg += roll_dice(6, attack.num_d6, rng);
    dmg += roll_dice(8, attack.num_d8, rng);
    dmg += roll_dice(10, attack.num_d10, rng);
    dmg += roll_dice(12, attack.num_d12, rng);
    dmg += roll_dice(20, attack.num_d20, rng);
    dmg
}

fn attack(attacker: &Creature, defender: &Creature, rng: &mut ThreadRng) -> i64 {
    let num_attacks = if attacker.stats.num_attacks > 0 {
        attacker.stats.num_attacks
    } else {
        1
    };
    let mut total = 0;
    for _ in 0..num_attacks {
        let attack = attacker.stats.attacks.choose(rng).unwrap();
        if hits(attack, defender.stats, rng) {
            let dmg = damage(attack, rng);
            println!(
                "{} ({}) does {} damage from {}",
                attacker.stats.name, attacker.id, dmg, attack.name
            );
            total += dmg;
        } else {
            println!("{} misses with {}", attacker.stats.name, attack.name);
        }
    }
    total
}

pub fn fight(creature1_stats: &StatBlock, creature2_stats: &StatBlock) -> Option<String> {
    let mut creature1 = Creature::new(creature1_stats, 1, 1);
    let mut creature2 = Creature::new(creature2_stats, 2, 2);
    let mut thread_rng = rand::thread_rng();
    let rng = &mut thread_rng;
    loop {
        if creature1.hp <= 0 {
            println!("{} won!", creature2.stats.name);
            break;
        }
        creature2.hp -= attack(&creature1, &creature2, rng);
        if creature2.hp <= 0 {
            println!("{} won!", creature1.stats.name);
            break;
        }
        creature1.hp -= attack(&creature2, &creature1, rng);
    }
    Some(String::from("Done"))
}

pub fn fight_teams(team1_stats: Vec<&StatBlock>, team2_stats: Vec<&StatBlock>) -> Option<String> {
    println!(
        "{} fighting {}",
        team1_stats
            .iter()
            .map(|&x| x.name.clone())
            .collect::<Vec<String>>()
            .join(", "),
        team2_stats
            .iter()
            .map(|&x| x.name.clone())
            .collect::<Vec<String>>()
            .join(", ")
    );
    let mut thread_rng = rand::thread_rng();
    let rng = &mut thread_rng;

    let mut creatures: Vec<Creature> = Vec::new();
    let mut team1: Vec<Creature> = Vec::new();
    let mut team2: Vec<Creature> = Vec::new();
    let mut id = 0;
    for stat in &team1_stats {
        let creature = Creature::new(stat, 1, id);
        team1.push(creature.clone());
        creatures.push(creature.clone());
        id += 1;
    }
    for stat in &team2_stats {
        let creature = Creature::new(stat, 2, id);
        creatures.push(creature.clone());
        team2.push(creature.clone());
        id += 1;
    }
    let len = creatures.len();
    let mut i = 0;
    let mut dead = (0, 0);
    loop {
        if dead.0 == team1.len() || dead.1 == team2.len() {
            break;
        }
        i = (i + 1) % len;
        let creatures_mut = &mut creatures;
        let damage;
        let target;
        {
            let creature = creatures_mut.get_mut(i).unwrap();
            if creature.hp <= 0 {
                continue;
            }
            let other_team = if creature.team == 1 { &team2 } else { &team1 };
            target = other_team.choose(rng).unwrap();
            damage = attack(creature, target, rng);
        }
        for creature in creatures_mut.iter_mut() {
            if creature.id == target.id {
                creature.hp -= damage;
                if creature.hp <= 0 {
                    println!("{} ({}) died!", creature.stats.name, creature.id);
                    dead = if creature.team == 1 {
                        (dead.0 + 1, dead.1)
                    } else {
                        (dead.0, dead.1 + 1)
                    };
                }
            }
        }
    }
    if dead.0 == team1.len() {
        println!("Team 2 won!");
    } else if dead.1 == team2.len() {
        println!("Team 1 won!");
    }
    Some(String::from("Done"))
}

