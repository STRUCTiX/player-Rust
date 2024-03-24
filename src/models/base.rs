use serde::Deserialize;
use crate::models::position::Position;
use crate::models::progress::Progress;

#[derive(Deserialize)]
pub struct Base {
    pub position: Position,
    pub uid: u32,
    pub player: u32,
    pub population: u32,
    pub level: u32,
    #[serde(rename="unitsUntilUpgrade")]
    pub units_until_upgrade: Progress,
}