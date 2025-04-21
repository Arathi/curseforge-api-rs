#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct GetModFilesParameters {
    //
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SearchModsParameters {
    game_id: u32,
    class_id: Option<u32>,
    category_id: Option<u32>,
    game_version: Option<String>,
    search_filter: Option<String>,
    sort_field: Option<SortField>,
    sort_order: Option<SortOrder>,
    mod_loader_type: Option<ModLoaderType>,
    game_version_type_id: Option<u32>,
    slug: Option<String>,
    index: Option<u32>,
    page_size: Option<u32>,
}

#[derive(Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub enum SortField {
    Featured = 1,
    Popularity = 2,
    LastUpdated = 3,
    Name = 4,
    Author = 5,
    TotalDownloads = 6,
    Category = 7,
    GameVersion = 8,
    EarlyAccess = 9,
    FeaturedReleased = 10,
    ReleasedDate = 11,
    Rating = 12,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum SortOrder {
    #[allow(non_camel_case_types)]
    asc,
    #[allow(non_camel_case_types)]
    desc,
}

#[derive(Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub enum ModLoaderType {
    Any = 0,
    Forge = 1,
    Cauldron = 2,
    LiteLoader = 3,
    Fabric = 4,
    Quilt = 5,
    NeoForge = 6,
}
