// Chris de la Iglesia, 2019

extern crate rand;
extern crate serde;
extern crate wasm_bindgen;

use rand::prelude::*;
use rand::Rng;
use rand::seq::SliceRandom;
use serde::Deserialize;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::result::Result;
use std::vec::Vec;
use wasm_bindgen::prelude::*;
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
    #[serde(default)]
    dex_mod: i64,
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

fn attack_team(attacker: &Creature, team: &mut Vec<Creature>, rng: &mut ThreadRng) -> Option<usize> {
    let target_i = rng.gen_range(0, team.len());
    let target = team.get_mut(target_i).unwrap();
    let damage = attack(attacker, target, rng);
    target.hp -= damage;
    if target.hp <= 0 {
        println!("{} ({}) died!", target.stats.name, target.id);
        return Some(target_i);
    }
    return None;
}

fn remove_from_initiative_order(initiative_order: &mut Vec<(usize,usize)>, team: usize, team_i: usize) {
    for i in 0..initiative_order.len() {
        let (other_team, other_team_i) = initiative_order.get(i).unwrap();
        if *other_team == team && *other_team_i == team_i {
            initiative_order.remove(i);
            break;
        }
    }
}

fn roll_initiative(creature: &Creature, rng: &mut ThreadRng) -> i64 {
    return roll_dice(20, 1, rng) + creature.stats.dex_mod;
}

pub fn fight(creature1_stats: &StatBlock, creature2_stats: &StatBlock) -> Vec<String> {
    let mut output = Vec::new();
    let mut creature1 = Creature::new(creature1_stats, 1, 1);
    let mut creature2 = Creature::new(creature2_stats, 2, 2);
    let mut thread_rng = rand::thread_rng();
    let rng = &mut thread_rng;
    loop {
        if creature1.hp <= 0 {
            output.push(format!("{} won!", creature2.stats.name));
            break;
        }
        creature2.hp -= attack(&creature1, &creature2, rng);
        if creature2.hp <= 0 {
            output.push(format!("{} won!", creature1.stats.name));
            break;
        }
        creature1.hp -= attack(&creature2, &creature1, rng);
    }
    output
}

pub fn fight_teams<'a>(
    team1: &mut Vec<Creature<'a>>,
    team2: &mut Vec<Creature<'a>>,
) -> Option<i64> {
    println!(
        "{} fighting {}",
        team1
            .iter()
            .map(|x| x.stats.name.clone())
            .collect::<Vec<String>>()
            .join(", "),
        team2
            .iter()
            .map(|x| x.stats.name.clone())
            .collect::<Vec<String>>()
            .join(", ")
    );
    let mut thread_rng = rand::thread_rng();
    let rng = &mut thread_rng;

    let team1_length = team1.len();
    let team2_length = team2.len();

    let mut initiative_rolls = HashMap::new();
    let mut initiative_order: Vec<(usize, usize)> = Vec::new();
    for i in 0..team1_length {
        initiative_rolls.insert((1, i), roll_initiative(team1.get(i).unwrap(), rng));
        initiative_order.push((1, i));
    }
    for i in 0..team2_length {
        initiative_rolls.insert((2, i), roll_initiative(team2.get(i).unwrap(), rng));
        initiative_order.push((2, i));
    }
    initiative_order.sort_by_key(|value| initiative_rolls.get(value).unwrap());
    println!(
        "Fight order: {}",
        initiative_order
            .iter()
            .map(|value| {
                let team = value.0;
                let index = value.1;
                if team == 1 {
                    team1.get(index).unwrap().stats.name.clone()
                } else {
                    team2.get(index).unwrap().stats.name.clone()
                }
            })
            .collect::<Vec<String>>()
            .join(", ")
    );

    let mut i = 0;
    let mut team1_left = team1.len();
    let mut team2_left = team2.len();
    while team1_left > 0 && team2_left > 0 {
        i = (i + 1) % initiative_order.len();
        let (team, team_i) = initiative_order.get(i).unwrap();
        if *team == 1 {
            let attacker = team1.get(*team_i).unwrap();
            match attack_team(attacker, team2, rng) {
                Some(enemy_i) => {
                    remove_from_initiative_order(&mut initiative_order, 2, enemy_i); 
                    team2_left -= 1;
                },
                _ => {}
            }
        } else {
            let attacker = team2.get(*team_i).unwrap();
            match attack_team(attacker, team1, rng) {
                Some(enemy_i) => {
                    remove_from_initiative_order(&mut initiative_order, 1, enemy_i);
                    team1_left -= 1;
                },
                _ => {}
            }
        }
    }
    if team1_left > 0 {
        println!("Team 1 won!");
        Some(2)
    } else if team2_left > 0 {
        println!("Team 2 won!");
        Some(1)
    } else {
        None
    }
}

fn compile_output(lines: Vec<String>) -> String {
    let mut output = String::new();
    for s in lines {
        output.push_str(s.as_str());
        output.push_str("\n");
    }
    output
}

#[wasm_bindgen]
pub fn fight_creature(creature1: &str, creature2: &str) -> String {
    let creature1_stats = StatBlock::from_str(creature1).unwrap();
    let creature2_stats = StatBlock::from_str(creature2).unwrap();
    compile_output(fight(&creature1_stats, &creature2_stats))
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(msg: &str) {
    alert(&("Message: ".to_owned() + msg));
}
