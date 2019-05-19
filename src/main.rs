// Chris de la Iglesia

extern crate rand;
extern crate serde;
extern crate serde_json;

use std::env;

fn main() -> Result<(),String> {
    let args: Vec<String> = env::args().collect();
    let creature1filename = &args[1];
    let creature2filename = &args[2];
    let creature1 = monsterattack::StatBlock::from_file(creature1filename)?;
    let creature2 = monsterattack::StatBlock::from_file(creature2filename)?;
    monsterattack::fight(&creature1, &creature2);
    Ok(())
}
