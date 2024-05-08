use serde::Deserialize;

#[derive(Deserialize)]
pub struct PathConfig {
    #[serde(rename="gracePeriod")]
    pub grace_period: u32, // time until groups of bits take damage
    #[serde(rename="deathRate")]
    pub death_rate: u32,   // number of units killed every tick after surpassing the grace period
}