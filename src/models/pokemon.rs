use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub height: u32,
    pub weight: u32,
    #[serde(default)]
    pub base_experience: Option<u32>,
    pub types: Vec<PokemonType>,
    pub stats: Vec<PokemonStat>,
    pub sprites: PokemonSprites,
} 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonType {
    pub slot: u8,
    #[serde(rename = "type")]
    pub type_info: TypeInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeInfo {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonStat {
    pub base_stat: u32,
    pub effort: u32,
    #[serde(rename = "stat")]
    pub stat_info: StatInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatInfo {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonSprites {
    #[serde(rename = "front_default")]
    pub front: Option<String>,
    #[serde(rename = "back_default")]
    pub back: Option<String>,
}