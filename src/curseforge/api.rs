use super::error::CurseForgeResult;
use super::schemas::Category;
use super::schemas::DataResponse;
use super::schemas::File;
use super::schemas::Game;
use super::schemas::Mod;
use super::schemas::{GameVersionType, GameVersionsByType};
use super::schemas::{GetModFilesParameters, SearchModsParameters};
use super::schemas::{ListResponse, PaginationResponse};

/// 游戏API
pub trait GamesApi {
    /// 获取游戏列表
    fn get_games(
        &self,
        index: Option<u32>,
        page_size: Option<u32>,
    ) -> CurseForgeResult<PaginationResponse<Game>>;

    /// 获取指定游戏
    fn get_game(&self, game_id: u32) -> CurseForgeResult<DataResponse<Game>>;

    /// 获取版本
    fn get_versions(&self, game_id: u32) -> CurseForgeResult<ListResponse<GameVersionsByType>>;

    /// 获取类型分组后的版本
    fn get_version_types(&self, game_id: u32) -> CurseForgeResult<ListResponse<GameVersionType>>;
}

/// 分类API
pub trait CategoriesApi {
    /// 获取分类
    fn get_categories(
        &self,
        game_id: u32,
        class_id: Option<u32>,
        classes_only: Option<bool>,
    ) -> CurseForgeResult<ListResponse<Category>>;
}

/// 模组API
pub trait ModsApi {
    /// 搜索模组
    fn search_mods(
        &self,
        params: SearchModsParameters,
    ) -> CurseForgeResult<PaginationResponse<Mod>>;

    /// 获取模组
    fn get_mod(&self, mod_id: u32) -> CurseForgeResult<DataResponse<Mod>>;
}

/// 模组文件API
pub trait FilesApi {
    /// 获取模组文件
    fn get_mod_file(&self, mod_id: u32, file_id: u32) -> CurseForgeResult<DataResponse<File>>;

    /// 搜索模组文件
    fn get_mod_files(
        &self,
        mod_id: u32,
        options: GetModFilesParameters,
    ) -> CurseForgeResult<PaginationResponse<File>>;
}
