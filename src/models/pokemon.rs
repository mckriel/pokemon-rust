use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub height: u32,
    pub weight: u32,
    pub types: Vec<PokemonType>,
    pub stats: Vec<PokemonStat>,
    pub sprites: PokemonSprites,
} 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonType {
    pub slot: u8,
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
    pub stat_info: StatInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatInfo {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonSprites {
    pub front: Option<String>,
    pub back: Option<String>,
}