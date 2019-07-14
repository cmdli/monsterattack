// Chris de la Iglesia

extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate wasm_bindgen;

use std::env;

fn create_team(
    stat: &monsterattack::StatBlock,
    num: i64,
    team_id: i64,
    start_id: i64,
) -> Vec<monsterattack::Creature> {
    let mut id = start_id;
    let mut team = Vec::new();
    for _ in 0..num {
        team.push(monsterattack::Creature::new(stat, team_id, id));
        id += 1;
    }
    team
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let creature1filename = &args[1];
    let num_team1 = args[2].parse::<i64>().unwrap();
    let creature2filename = &args[3];
    let num_team2 = args[4].parse::<i64>().unwrap();

    let mut team1won = 0;
    let mut team2won = 0;
    let stat1 = monsterattack::StatBlock::from_file(creature1filename)?;
    let stat2 = monsterattack::StatBlock::from_file(creature2filename)?;
    for _ in 0..1 {
        let mut team1: Vec<monsterattack::Creature> = create_team(&stat1, num_team1, 1, 0);
        let mut team2: Vec<monsterattack::Creature> = create_team(&stat2, num_team2, 2, num_team1);
        match monsterattack::fight_teams(&mut team1, &mut team2) {
            Some(1) => team1won += 1,
            Some(2) => team2won += 1,
            Some(x) => println!("Unexpected team: {}", x),
            None => println!("Error!"),
        }
    }
    println!("Team 1: {} Team 2: {}", team1won, team2won);
    Ok(())
}
