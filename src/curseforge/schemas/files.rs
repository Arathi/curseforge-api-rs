use super::mod_loader::ModLoaderType;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    id: u32,
    game_id: u32,
    mod_id: u32,
    // is_available: bool,
    display_name: String,
    file_name: String,
    release_type: FileReleaseType,
    file_status: FileStatus,
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
    dynamic_fields: HashMap<String, Value>,
}

impl File {
    pub fn md5(&self) -> Option<String> {
        let hash = self.hashes.iter().find(|hash| hash.algo == HashAlgo::Md5);
        match hash {
            Some(hash) => Some(hash.value.clone()),
            None => None,
        }
    }

    pub fn sha1(&self) -> Option<String> {
        let hash = self.hashes.iter().find(|hash| hash.algo == HashAlgo::Sha1);
        match hash {
            Some(hash) => Some(hash.value.clone()),
            None => None,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum FileStatus {
    Processing = 1,
    ChangesRequired = 2,
    UnderReview = 3,
    Approved = 4,
    Rejected = 5,
    MalwareDetected = 6,
    Deleted = 7,
    Archived = 8,
    Testing = 9,
    Released = 10,
    ReadyForReview = 11,
    Deprecated = 12,
    Baking = 13,
    AwaitingPublishing = 14,
    FailedPublishing = 15,
    Cooking = 16,
    Cooked = 17,
    UnderManualReview = 18,
    ScanningForMalware = 19,
    ProcessingFile = 20,
    PendingRelease = 21,
    ReadyForCooking = 22,
    PostProcessing = 23,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file() {
        let json = r#"{
    "id": 3835119,
    "gameId": 432,
    "modId": 245755,
    "isAvailable": true,
    "displayName": "waystones-forge-1.19-11.0.0.jar",
    "fileName": "waystones-forge-1.19-11.0.0.jar",
    "releaseType": 1,
    "fileStatus": 4,
    "hashes": [
        {
            "value": "679b3c87f9145400ffb7fa81b2f918443cba72df",
            "algo": 1
        },
        {
            "value": "591352d7c666d2607ad4c473bb48ad10",
            "algo": 2
        }
    ],
    "fileDate": "2022-06-16T20:25:04.053Z",
    "fileLength": 359351,
    "downloadCount": 252131,
    "downloadUrl": "https://edge.forgecdn.net/files/3835/119/waystones-forge-1.19-11.0.0.jar",
    "gameVersions": [
        "Forge",
        "1.19"
    ],
    "sortableGameVersions": [
        {
            "gameVersionName": "Forge",
            "gameVersionPadded": "0",
            "gameVersion": "",
            "gameVersionReleaseDate": "2022-10-01T00:00:00Z",
            "gameVersionTypeId": 68441
        },
        {
            "gameVersionName": "1.19",
            "gameVersionPadded": "0000000001.0000000019",
            "gameVersion": "1.19",
            "gameVersionReleaseDate": "2022-06-07T15:38:07.377Z",
            "gameVersionTypeId": 73407
        }
    ],
    "dependencies": [
        {
            "modId": 531761,
            "relationType": 3
        }
    ],
    "alternateFileId": 0,
    "isServerPack": false,
    "fileFingerprint": 2580803773,
    "modules": [
        {
            "name": "META-INF",
            "fingerprint": 2731717902
        },
        {
            "name": "net",
            "fingerprint": 2705334492
        },
        {
            "name": "assets",
            "fingerprint": 1478786455
        },
        {
            "name": "data",
            "fingerprint": 1311553820
        },
        {
            "name": "pack.mcmeta",
            "fingerprint": 2905512523
        },
        {
            "name": "waystones-icon.png",
            "fingerprint": 3621685521
        },
        {
            "name": "waystones.mixins.json",
            "fingerprint": 2087209328
        },
        {
            "name": "waystones.refmap.json",
            "fingerprint": 2378598686
        }
    ]
}"#;

        let result = serde_json::from_str(json).map_err(|e| {
            eprintln!("反序列化File出错：{}", e);
            e
        });
        assert!(result.is_ok());

        let file: File = result.unwrap();
        assert_eq!(file.id, 3835119);
        assert_eq!(file.game_id, 432);
        assert_eq!(file.mod_id, 245755);
        assert_eq!(file.display_name, "waystones-forge-1.19-11.0.0.jar");
        assert_eq!(file.release_type, FileReleaseType::Release);
        assert_eq!(file.file_status, FileStatus::Approved);
        assert_eq!(file.file_length, 359351);

        if let Some(hash) = file.sha1() {
            assert_eq!(hash, "679b3c87f9145400ffb7fa81b2f918443cba72df");
        }
        if let Some(hash) = file.md5() {
            assert_eq!(hash, "591352d7c666d2607ad4c473bb48ad10");
        }
    }
}
