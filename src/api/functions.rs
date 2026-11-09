use crate::api::types::GameInfo;

use crate::api::error::{Error, Result};

pub async fn get_game_infos() -> Result<GameInfo>
{
    Ok(reqwest::get("http://127.0.0.1:3073/api/v1/game-info").await?.json().await?)
}
