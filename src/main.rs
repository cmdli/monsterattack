// Chris de la Iglesia

extern crate rand;
extern crate serde;
extern crate serde_json;

use std::env;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let creature1filename = &args[1];
    let creature2filename = &args[2];
    let mut team2 = Vec::new();
    for _ in 0..2 {
        let creature2 = monsterattack::StatBlock::from_file(creature2filename)?; 
        team2.push(creature2);   
    }
    let creature1 = monsterattack::StatBlock::from_file(creature1filename)?;
    monsterattack::fight_teams(vec![&creature1], team2.iter().map(|x| x).collect());
    Ok(())
}
