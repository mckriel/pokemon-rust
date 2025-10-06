use crate::error::ApiError;
use crate::models::Pokemon;
use reqwest::Client;

pub struct PokeApiClient {
    client: Client,
    base_url: String,
}

impl PokeApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://pokeapi.co/api/v2".to_string(),
        }
    }

    pub async fn get_pokemon(&self, name_or_id: &str) -> Result<Pokemon, ApiError> {
        let url = format!("{}/pokemon/{}", self.base_url, name_or_id);

        let response = self.client
            .get(&url)
            .send()
            .await?;

        if response.status() == 404 {
            return Err(ApiError::PokemonNotFound {
                name: name_or_id.to_string()
            });
        }

        let pokemon = response
            .json::<Pokemon>()
            .await?;

        Ok(pokemon)
    }
}