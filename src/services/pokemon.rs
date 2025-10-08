use crate::client::PokeApiClient;
use crate::error::ApiError;
use crate::models::Pokemon;

pub struct PokemonService {
    client: PokeApiClient,
}

impl PokemonService {
    pub fn new() -> Self {
        Self {
            client: PokeApiClient::new(),
        }
    }

    pub async fn get_pokemon(&self, name_or_id: &str) -> Result<Pokemon, ApiError> {
        if name_or_id.trim().is_empty() {
            return Err(ApiError::bad_request("Pokemon name or id cannot be empty"))
        }

        let normalized_name = name_or_id.trim().to_lowercase();

        self.client.get_pokemon(&normalized_name).await
    }
}