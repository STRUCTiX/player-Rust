use serde::Deserialize;

#[derive(Deserialize)]
pub struct Progress {
    pub distance: u32, // distance between the bases
    pub traveled: u32, // distance already traveled
}

