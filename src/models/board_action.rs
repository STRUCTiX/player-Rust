use serde::Deserialize;

#[derive(Deserialize)]
pub struct BoardAction {
    pub src: u32,            // uid of source base
    pub dest: u32,           // uid of destination base
    pub amount: u32,         // number of bits moved
    #[serde(rename="maxPopulation")]
    pub max_population: u32, // number of sustainable bits
    #[serde(rename="upgradeCost")]
    pub upgrade_cost: u32,   // bits required to unlock this upgrade
    #[serde(rename="spawnRate")]
    pub spawn_rate: u32,     // number uf bits spawned per tick
}