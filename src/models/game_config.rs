use crate::models::base_level::BaseLevel;
use crate::models::path_config::PathConfig;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct GameConfig {
    base_levels: Vec<BaseLevel>, // all available base levels
    paths: PathConfig,           // settings containing paths between bases
}

