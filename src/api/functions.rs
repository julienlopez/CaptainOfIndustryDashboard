use crate::api::types::GameInfo;

use crate::api::error::{Error, Result};

pub async fn get_game_infos() -> Result<GameInfo>
{
    Err(Error::Message("To be implemented".into()))
}
