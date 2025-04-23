use super::mod_loader::ModLoaderType;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    id: u32,
    game_id: u32,
    mod_id: u32,
    is_available: bool,
    display_name: String,
    file_name: String,
    // release_type: u32,
    // file_status: u32,
    hashes: Vec<FileHash>,
    file_date: String,
    file_length: u64,
    download_count: u64,
    file_size_on_disk: Option<u64>,
    download_url: String,
    game_versions: Vec<String>,
    // sortableGameVersions
    dependencies: Vec<FileDependency>,
    // exposeAsAlternative
    // parentProjectFileId	integer(int32)¦null	none
    // alternateFileId	integer(int32)¦null	none
    // isServerPack	boolean¦null	none
    // serverPackFileId	integer(int32)¦null	none
    // isEarlyAccessContent	boolean¦null	none
    // earlyAccessEndDate	string(date-time)¦null	none
    // fileFingerprint	integer(int64)	none
    // modules	[FileModule]	nSelf::one
    #[serde(flatten)]
    unknown_fields: HashMap<String, Value>,
}

pub struct FileIndex {
    game_version: String,
    file_id: u32,
    filename: String,
    release_type: FileReleaseType,
    game_version_type: Option<u32>,
    mod_loader: ModLoaderType,
}

#[derive(Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum FileReleaseType {
    Release = 1,
    Beta = 2,
    Alpha = 3,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FileHash {
    value: String,
    algo: HashAlgo,
}

#[derive(Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum HashAlgo {
    Sha1 = 1,
    Md5 = 2,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FileDependency {
    mod_id: u32,
    relation_type: FileRelationType,
}

#[derive(Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum FileRelationType {
    EmbeddedLibrary = 1,
    OptionalDependency = 2,
    RequiredDependency = 3,
    Tool = 4,
    Incompatible = 5,
    Include = 6,
}
