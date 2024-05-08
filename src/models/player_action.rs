use serde::Serialize;

#[derive(Serialize)]
pub struct PlayerAction {
    pub src: u32,    // uid of source base
    pub dest: u32,   // uid of destination base
    pub amount: u32, // number of bits moved
}

