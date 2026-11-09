use serde::Deserialize;


#[derive(Debug,  Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameInfo
{
    pub date: String,
    pub tick: u64,
    pub is_paused: bool,
    pub speed: u8,
    pub game_id: String,
    pub session_id: String
}
