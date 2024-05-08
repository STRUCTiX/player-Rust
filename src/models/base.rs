use crate::models::position::Position;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base {
    pub position: Position, // position of the base
    pub uid: u32,           // uid of the base
    pub player: u32,        // owner of the base
    pub population: u32,    // current population of the base
    pub level: u32,         // level of the base
    #[serde(rename = "unitsUntilUpgrade")]
    pub units_until_upgrade: u32, // number of units required to upgrade
}

