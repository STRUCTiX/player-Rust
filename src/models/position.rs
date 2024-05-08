use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Position {
    pub x: i32, // x coordinate
    pub y: i32, // y coordinate
    pub z: i32, // y coordinate
}

