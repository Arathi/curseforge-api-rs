#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GameVersionsByType {
    #[serde(rename = "type")]
    pub version_type: i32,

    #[serde(rename = "versions")]
    pub versions: Vec<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GameVersionType {
    id: i32,
    game_id: i32,
    name: String,
    slug: String,
    is_syncable: bool,
    status: GameVersionTypeStatus,
}

#[derive(Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub enum GameVersionTypeStatus {
    Normal = 1,
    Deleted = 2,
}
