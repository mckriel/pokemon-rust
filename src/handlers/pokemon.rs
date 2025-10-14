use crate::error::ApiError;
use crate::services::PokemonService;
use crate::models::Pokemon;
use axum:: {
    extract::{Path, State},
    response::Json,
};
use std::sync::Arc;


pub async fn get_pokemon( 
    State(service): State<Arc<PokemonService>>, 
    Path(name_or_id): Path<String> 
) -> Result<Json<Pokemon>, ApiError> {
    let pokemon = service.get_pokemon(&name_or_id).await?;
    Ok(Json(pokemon))
}