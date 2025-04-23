/// 游戏
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    /// 编号
    pub id: i32,

    /// 名称
    pub name: String,

    /// Slug
    pub slug: String,

    // 修改时间
    pub date_modified: String,

    /// 游戏资源
    pub assets: GameAssets,

    /// 状态
    pub status: CoreStatus,

    /// API状态
    pub api_status: CoreApiStatus,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameAssets {
    icon_url: String,
    tile_url: String,
    cover_url: String,
}

#[derive(Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum CoreStatus {
    Draft = 1,
    Test = 2,
    PendingReview = 3,
    Rejected = 4,
    Approved = 5,
    Live = 6,
}

#[derive(Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum CoreApiStatus {
    Private = 1,
    Public = 2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_minecraft() {
        let json = r#"{
    "id": 432,
    "name": "Minecraft",
    "slug": "minecraft",
    "dateModified": "2024-08-29T09:54:14.45Z",
    "assets": {
        "iconUrl": "https://media.forgecdn.net/avatars/1070/13/638605220544214742.webp",
        "tileUrl": "https://media.forgecdn.net/game-box-art/432_2734e7e3-c584-46e0-81ca-75973b380c9b.webp",
        "coverUrl": "https://media.forgecdn.net/game-covers/432_5076448d-feda-439b-9d63-ce86b70459ef.webp"
    },
    "status": 6,
    "apiStatus": 2
}"#;

        let result = serde_json::from_str(json);
        assert!(result.is_ok());

        let minecraft: Game = result.unwrap();
        assert_eq!(minecraft.id, 432);
        assert_eq!(minecraft.name, "Minecraft");
        assert_eq!(minecraft.slug, "minecraft");
        assert_eq!(minecraft.status, CoreStatus::Live);
        assert_eq!(minecraft.api_status, CoreApiStatus::Public);

        println!(
            "minecraft = {}",
            serde_json::to_string_pretty(&minecraft).unwrap()
        );
    }
}
