use crate::models::progress::Progress;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Clone, Copy)]
pub struct BoardAction {
    pub src: u32,           // uid of source base
    pub dest: u32,          // uid of destination base
    pub amount: u32,        // number of bits moved
    pub uuid: Uuid,         // uuid of the action
    pub player: u32,        // id of the player who took the action
    pub progress: Progress, // progress off the action
}

