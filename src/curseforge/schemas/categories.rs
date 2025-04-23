/// 种类（category）
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    /// 种类编号
    pub id: u32,

    /// 游戏编号
    pub game_id: u32,

    /// 名称
    pub name: String,

    /// Slug
    pub slug: String,

    /// URL
    pub url: String,

    /// 图标URL
    pub icon_url: String,

    /// 修改时间
    pub date_modified: String,

    /// 是否为分类（class）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_class: Option<bool>,

    /// 上级分类编号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_id: Option<u32>,

    /// 种类编号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_category_id: Option<u32>,

    /// 显示顺序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_index: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mods() {
        let json = r#"{
            "id": 6,
            "gameId": 432,
            "name": "Mods",
            "slug": "mc-mods",
            "url": "https://www.curseforge.com/minecraft/mc-mods",
            "iconUrl": "https://media.forgecdn.net/avatars/52/101/636111139584399357.png",
            "dateModified": "2023-08-05T12:43:53.26Z",
            "isClass": true,
            "displayIndex": 1
        }"#;

        let result = serde_json::from_str(json);
        assert!(result.is_ok());

        let mods: Category = result.unwrap();
        assert_eq!(mods.id, 6);
        assert_eq!(mods.game_id, 432);
        assert_eq!(mods.name, "Mods");
        assert_eq!(mods.slug, "mc-mods");

        assert!(mods.is_class.is_some());
        assert_eq!(mods.is_class.unwrap(), true);

        println!("{}", serde_json::to_string_pretty(&mods).unwrap());
    }

    #[test]
    fn test_parse_create() {
        let json = r#"{
            "id": 6484,
            "gameId": 432,
            "name": "Create",
            "slug": "create",
            "url": "https://www.curseforge.com/minecraft/mc-mods/mc-addons/create",
            "iconUrl": "https://media.forgecdn.net/avatars/834/490/638225251884617502.png",
            "dateModified": "2023-06-16T15:13:08.513Z",
            "classId": 6,
            "parentCategoryId": 426,
            "displayIndex": 2
        }"#;

        let result = serde_json::from_str(json);
        assert!(result.is_ok());

        let create: Category = result.unwrap();
        assert_eq!(create.id, 6484);
        assert_eq!(create.game_id, 432);
        assert_eq!(create.name, "Create");
        assert_eq!(create.slug, "create");

        assert!(create.is_class.is_none());

        assert!(create.class_id.is_some());
        assert_eq!(create.class_id.unwrap(), 6);

        assert!(create.parent_category_id.is_some());
        assert_eq!(create.parent_category_id.unwrap(), 426);

        println!("{}", serde_json::to_string_pretty(&create).unwrap());
    }
}
