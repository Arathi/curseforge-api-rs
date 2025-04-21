#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Game {
    pub id: i32,

    pub name: String,

    pub slug: String,

    #[serde(rename = "dateModified")]
    pub date_modified: String,

    pub assets: GameAssets,

    pub status: CoreStatus,

    pub api_status: CoreApiStatus,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GameAssets {
    icon_url: String,
    tile_url: String,
    cover_url: String,
}

#[derive(Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub enum CoreStatus {
    Draft = 1,
    Test = 2,
    PendingReview = 3,
    Rejected = 4,
    Approved = 5,
    Live = 6,
}

#[derive(Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub enum CoreApiStatus {
    Private = 1,
    Public = 2,
}
