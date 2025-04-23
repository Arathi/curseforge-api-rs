use std::collections::HashMap;

use serde_json::Value;

use super::Category;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mod {
    /// 编号
    id: u32,

    /// 游戏编号
    game_id: u32,

    /// 名称
    name: String,

    /// Slug
    slug: String,

    /// 链接
    // links: ModLinks,

    /// 概要
    summary: String,

    /// 状态
    status: ModStatus,

    /// 下载量
    download_count: u64,

    // is_featured
    primary_category_id: u32,

    /// 种类/分类
    categories: Vec<Category>,

    /// 分类编号
    class_id: Option<u32>,

    /// 作者
    authors: Vec<ModAuthor>,

    /// Logo
    logo: ModAsset,

    /// 截图
    // screenshots: Vec<ModAsset>,

    /// 主文件
    main_file_id: u32,

    /// 最新文件
    // latest_files: Vec<File>,

    /// 最新文件索引
    // latest_files_indexes: Vec<FileIndex>,

    /// 最新预览版文件索引
    // latest_early_access_files_indexes: Vec<FileIndex>,

    /// 创建时间
    date_created: String,

    /// 修改时间
    date_modified: String,

    /// 发布时间
    date_released: String,

    /// 是否允许模组分发
    // allow_mod_distribution: bool,

    /// 游戏流行度排名
    // game_popularity_rank

    /// 是否允许搜索
    // is_available: bool,

    /// 评论数量？
    // thumbs_up_count: u32,

    /// 评分
    // rating: Option<f32>,

    /// 未知字段
    #[serde(flatten)]
    dynamic_fields: HashMap<String, Value>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModLinks {
    /// 网站
    website_url: String,

    /// Wiki
    wiki_url: String,

    /// 问题汇报
    issues_url: String,

    /// 源码
    source_url: String,
}

#[derive(Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum ModStatus {
    New = 1,
    ChangesRequired = 2,
    UnderSoftReview = 3,
    Approved = 4,
    Rejected = 5,
    ChangesMade = 6,
    Inactive = 7,
    Abandoned = 8,
    Deleted = 9,
    UnderReview = 10,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModAuthor {
    /// 编号
    id: u32,

    /// 名称
    name: String,

    /// URL
    url: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModAsset {
    /// 编号
    id: u32,

    /// 模组编号
    mod_id: u32,

    /// 标题
    title: String,

    /// 描述
    description: String,

    /// 缩略图URL
    thumbnail_url: String,

    /// 下载地址
    url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_jei() {
        //#region JSON
        let json = r#"{
    "screenshots": [
        {
            "id": 31420,
            "modId": 238222,
            "title": "Itemlist Edit Mode",
            "description": "",
            "thumbnailUrl": "https://media.forgecdn.net/attachments/thumbnails/31/420/310/172/tgafkma.png",
            "url": "https://media.forgecdn.net/attachments/31/420/tgafkma.png"
        },
        {
            "id": 31419,
            "modId": 238222,
            "title": "Potions",
            "description": "",
            "thumbnailUrl": "https://media.forgecdn.net/attachments/thumbnails/31/419/310/172/t7f7jh6.png",
            "url": "https://media.forgecdn.net/attachments/31/419/t7f7jh6.png"
        },
        {
            "id": 31418,
            "modId": 238222,
            "title": "Big Screen Support",
            "description": "",
            "thumbnailUrl": "https://media.forgecdn.net/attachments/thumbnails/31/418/310/172/9lngh5f.png",
            "url": "https://media.forgecdn.net/attachments/31/418/9lngh5f.png"
        },
        {
            "id": 31417,
            "modId": 238222,
            "title": "Recipe Completion",
            "description": "",
            "thumbnailUrl": "https://media.forgecdn.net/attachments/thumbnails/31/417/310/172/thzzdin.png",
            "url": "https://media.forgecdn.net/attachments/31/417/thzzdin.png"
        }
    ],
    "id": 238222,
    "gameId": 432,
    "name": "Just Enough Items (JEI)",
    "slug": "jei",
    "links": {
        "websiteUrl": "https://www.curseforge.com/minecraft/mc-mods/jei",
        "wikiUrl": "",
        "issuesUrl": "https://github.com/mezz/JustEnoughItems/issues?q=is%3Aissue",
        "sourceUrl": "https://github.com/mezz/JustEnoughItems"
    },
    "summary": "View Items and Recipes",
    "status": 4,
    "downloadCount": 430300183,
    "isFeatured": false,
    "primaryCategoryId": 423,
    "categories": [
        {
            "id": 421,
            "gameId": 432,
            "name": "API and Library",
            "slug": "library-api",
            "url": "https://www.curseforge.com/minecraft/mc-mods/library-api",
            "iconUrl": "https://media.forgecdn.net/avatars/6/36/635351496947765531.png",
            "dateModified": "2014-05-23T03:21:44.06Z",
            "isClass": false,
            "classId": 6,
            "parentCategoryId": 6
        },
        {
            "id": 423,
            "gameId": 432,
            "name": "Map and Information",
            "slug": "map-information",
            "url": "https://www.curseforge.com/minecraft/mc-mods/map-information",
            "iconUrl": "https://media.forgecdn.net/avatars/6/38/635351497437388438.png",
            "dateModified": "2014-05-08T17:42:23.74Z",
            "isClass": false,
            "classId": 6,
            "parentCategoryId": 6
        }
    ],
    "classId": 6,
    "authors": [
        {
            "id": 17072262,
            "name": "mezz",
            "url": "https://www.curseforge.com/members/mezz",
            "avatarUrl": "https://static-cdn.jtvnw.net/jtv_user_pictures/ce4a8a5fe354959b-profile_image-150x150.png"
        }
    ],
    "logo": {
        "id": 29069,
        "modId": 238222,
        "title": "635838945588716414.jpeg",
        "description": "",
        "thumbnailUrl": "https://media.forgecdn.net/avatars/thumbnails/29/69/256/256/635838945588716414.jpeg",
        "url": "https://media.forgecdn.net/avatars/29/69/635838945588716414.jpeg"
    },
    "mainFileId": 6075247,
    "latestFiles": [
        {
            "id": 3040523,
            "gameId": 432,
            "modId": 238222,
            "isAvailable": true,
            "displayName": "jei_1.12.2-4.16.1.301.jar",
            "fileName": "jei_1.12.2-4.16.1.301.jar",
            "releaseType": 1,
            "fileStatus": 4,
            "hashes": [
                {
                    "value": "3045e8440ea44071d8b83c4e7b3c190348fdc527",
                    "algo": 1
                },
                {
                    "value": "1dee4be93d666e2228039c551e927b35",
                    "algo": 2
                }
            ],
            "fileDate": "2020-08-24T01:01:39.123Z",
            "fileLength": 653211,
            "downloadCount": 26341234,
            "downloadUrl": "https://edge.forgecdn.net/files/3040/523/jei_1.12.2-4.16.1.301.jar",
            "gameVersions": [
                "1.12.2"
            ],
            "sortableGameVersions": [
                {
                    "gameVersionName": "1.12.2",
                    "gameVersionPadded": "0000000001.0000000012.0000000002",
                    "gameVersion": "1.12.2",
                    "gameVersionReleaseDate": "2017-09-18T05:00:00Z",
                    "gameVersionTypeId": 628
                }
            ],
            "dependencies": [],
            "alternateFileId": 0,
            "isServerPack": false,
            "fileFingerprint": 3089143260,
            "modules": [
                {
                    "name": "META-INF",
                    "fingerprint": 2236405288
                },
                {
                    "name": "mezz",
                    "fingerprint": 2222830911
                },
                {
                    "name": "pack.mcmeta",
                    "fingerprint": 1488642189
                },
                {
                    "name": "mcmod.info",
                    "fingerprint": 3528499262
                },
                {
                    "name": "assets",
                    "fingerprint": 9943101
                }
            ]
        },
        {
            "id": 3272039,
            "gameId": 432,
            "modId": 238222,
            "isAvailable": true,
            "displayName": "jei-1.13.2-5.0.0.31.jar",
            "fileName": "jei-1.13.2-5.0.0.31.jar",
            "releaseType": 3,
            "fileStatus": 4,
            "hashes": [
                {
                    "value": "aa15cdea079db8b91d75e3c68216df80a70545d8",
                    "algo": 1
                },
                {
                    "value": "1ee1f4fb4c6e199c02c7d15cbd0d2c8a",
                    "algo": 2
                }
            ],
            "fileDate": "2021-04-11T03:49:47.687Z",
            "fileLength": 690802,
            "downloadCount": 28655,
            "downloadUrl": "https://edge.forgecdn.net/files/3272/39/jei-1.13.2-5.0.0.31.jar",
            "gameVersions": [
                "1.13.2"
            ],
            "sortableGameVersions": [
                {
                    "gameVersionName": "1.13.2",
                    "gameVersionPadded": "0000000001.0000000013.0000000002",
                    "gameVersion": "1.13.2",
                    "gameVersionReleaseDate": "2018-10-22T00:00:00Z",
                    "gameVersionTypeId": 55023
                }
            ],
            "dependencies": [],
            "alternateFileId": 0,
            "isServerPack": false,
            "fileFingerprint": 2700304635,
            "modules": [
                {
                    "name": "META-INF",
                    "fingerprint": 1102858494
                },
                {
                    "name": "mezz",
                    "fingerprint": 2811918946
                },
                {
                    "name": "pack.mcmeta",
                    "fingerprint": 3652707984
                },
                {
                    "name": "assets",
                    "fingerprint": 88833534
                }
            ]
        },
        {
            "id": 4087656,
            "gameId": 432,
            "modId": 238222,
            "isAvailable": true,
            "displayName": "jei-1.19.2-fabric-11.4.0.286.jar",
            "fileName": "jei-1.19.2-fabric-11.4.0.286.jar",
            "releaseType": 3,
            "fileStatus": 4,
            "hashes": [
                {
                    "value": "f4c77ecd8b897a12c2c8a26350d93a93322a8bcd",
                    "algo": 1
                },
                {
                    "value": "cbf23483d172a38b71419100e227f017",
                    "algo": 2
                }
            ],
            "fileDate": "2022-11-15T02:14:24.907Z",
            "fileLength": 1068892,
            "downloadCount": 90125,
            "downloadUrl": "https://edge.forgecdn.net/files/4087/656/jei-1.19.2-fabric-11.4.0.286.jar",
            "gameVersions": [
                "Fabric",
                "1.19.2"
            ],
            "sortableGameVersions": [
                {
                    "gameVersionName": "Fabric",
                    "gameVersionPadded": "0",
                    "gameVersion": "",
                    "gameVersionReleaseDate": "2022-09-01T00:00:00Z",
                    "gameVersionTypeId": 68441
                },
                {
                    "gameVersionName": "1.19.2",
                    "gameVersionPadded": "0000000001.0000000019.0000000002",
                    "gameVersion": "1.19.2",
                    "gameVersionReleaseDate": "2022-08-05T14:12:22.413Z",
                    "gameVersionTypeId": 73407
                }
            ],
            "dependencies": [],
            "alternateFileId": 0,
            "isServerPack": false,
            "fileFingerprint": 1613607509,
            "modules": [
                {
                    "name": "META-INF",
                    "fingerprint": 2822576509
                },
                {
                    "name": "jei.accesswidener",
                    "fingerprint": 3441454662
                },
                {
                    "name": "assets",
                    "fingerprint": 224894697
                },
                {
                    "name": "fabric.mod.json",
                    "fingerprint": 1230897332
                },
                {
                    "name": "pack.mcmeta",
                    "fingerprint": 1550930300
                },
                {
                    "name": "jei.mixins.json",
                    "fingerprint": 623960849
                },
                {
                    "name": "jei-icon.png",
                    "fingerprint": 2007185424
                },
                {
                    "name": "jei-1.19.2-fabric-refmap.json",
                    "fingerprint": 1412800118
                },
                {
                    "name": "mezz",
                    "fingerprint": 2346665333
                }
            ]
        },
        {
            "id": 4087658,
            "gameId": 432,
            "modId": 238222,
            "isAvailable": true,
            "displayName": "jei-1.19.2-forge-11.4.0.286.jar",
            "fileName": "jei-1.19.2-forge-11.4.0.286.jar",
            "releaseType": 3,
            "fileStatus": 4,
            "hashes": [
                {
                    "value": "3bab715ae0f56e1b9a3e1ebfb5e9bb3f677e3711",
                    "algo": 1
                },
                {
                    "value": "0d458f02d611eafbf29ae36010d03130",
                    "algo": 2
                }
            ],
            "fileDate": "2022-11-15T02:14:49.103Z",
            "fileLength": 1046401,
            "downloadCount": 662161,
            "downloadUrl": "https://edge.forgecdn.net/files/4087/658/jei-1.19.2-forge-11.4.0.286.jar",
            "gameVersions": [
                "1.19.2",
                "Forge"
            ],
            "sortableGameVersions": [
                {
                    "gameVersionName": "1.19.2",
                    "gameVersionPadded": "0000000001.0000000019.0000000002",
                    "gameVersion": "1.19.2",
                    "gameVersionReleaseDate": "2022-08-05T14:12:22.413Z",
                    "gameVersionTypeId": 73407
                },
                {
                    "gameVersionName": "Forge",
                    "gameVersionPadded": "0",
                    "gameVersion": "",
                    "gameVersionReleaseDate": "2022-10-01T00:00:00Z",
                    "gameVersionTypeId": 68441
                }
            ],
            "dependencies": [],
            "alternateFileId": 0,
            "isServerPack": false,
            "fileFingerprint": 4135756105,
            "modules": [
                {
                    "name": "META-INF",
                    "fingerprint": 2624845708
                },
                {
                    "name": "mezz",
                    "fingerprint": 1839722990
                },
                {
                    "name": "jei-icon.png",
                    "fingerprint": 2007185424
                },
                {
                    "name": "pack.mcmeta",
                    "fingerprint": 1550930300
                },
                {
                    "name": "assets",
                    "fingerprint": 224894697
                }
            ]
        },
        {
            "id": 5060339,
            "gameId": 432,
            "modId": 238222,
            "isAvailable": true,
            "displayName": "jei_1.12.2-4.16.1.1004.jar",
            "fileName": "jei_1.12.2-4.16.1.1004.jar",
            "releaseType": 2,
            "fileStatus": 4,
            "hashes": [
                {
                    "value": "f64479688b83762482f8675b3a23223389e6c693",
                    "algo": 1
                },
                {
                    "value": "84c1540e5888f2af4c53709f6649ed2d",
                    "algo": 2
                }
            ],
            "fileDate": "2024-01-26T06:10:11.413Z",
            "fileLength": 654181,
            "downloadCount": 84051,
            "downloadUrl": "https://edge.forgecdn.net/files/5060/339/jei_1.12.2-4.16.1.1004.jar",
            "gameVersions": [
                "1.12.2"
            ],
            "sortableGameVersions": [
                {
                    "gameVersionName": "1.12.2",
                    "gameVersionPadded": "0000000001.0000000012.0000000002",
                    "gameVersion": "1.12.2",
                    "gameVersionReleaseDate": "2017-09-18T05:00:00Z",
                    "gameVersionTypeId": 628
                }
            ],
            "dependencies": [],
            "alternateFileId": 0,
            "isServerPack": false,
            "fileFingerprint": 2559345639,
            "modules": [
                {
                    "name": "META-INF",
                    "fingerprint": 3367829501
                },
                {
                    "name": "mezz",
                    "fingerprint": 2795317040
                },
                {
                    "name": "pack.mcmeta",
                    "fingerprint": 1488642189
                },
                {
                    "name": "assets",
                    "fingerprint": 2844848637
                },
                {
                    "name": "mcmod.info",
                    "fingerprint": 718309853
                }
            ]
        },
        {
            "id": 5846848,
            "gameId": 432,
            "modId": 238222,
            "isAvailable": true,
            "displayName": "jei-1.20.4-neoforge-17.3.1.5.jar",
            "fileName": "jei-1.20.4-neoforge-17.3.1.5.jar",
            "releaseType": 2,
            "fileStatus": 4,
            "hashes": [
                {
                    "value": "f0aef5338cdf210aad0018caa4c96e255bc34a11",
                    "algo": 1
                },
                {
                    "value": "3bb3a1bb63af04d3ec1f206e5554abd5",
                    "algo": 2
                }
            ],
            "fileDate": "2024-10-26T08:18:13.277Z",
            "fileLength": 1094333,
            "downloadCount": 0,
            "fileSizeOnDisk": 2429153,
            "downloadUrl": "https://edge.forgecdn.net/files/5846/848/jei-1.20.4-neoforge-17.3.1.5.jar",
            "gameVersions": [
                "NeoForge",
                "1.20.4"
            ],
            "sortableGameVersions": [
                {
                    "gameVersionName": "NeoForge",
                    "gameVersionPadded": "0",
                    "gameVersion": "",
                    "gameVersionReleaseDate": "2023-07-25T00:00:00Z",
                    "gameVersionTypeId": 68441
                },
                {
                    "gameVersionName": "1.20.4",
                    "gameVersionPadded": "0000000001.0000000020.0000000004",
                    "gameVersion": "1.20.4",
                    "gameVersionReleaseDate": "2023-12-07T15:17:47.907Z",
                    "gameVersionTypeId": 75125
                }
            ],
            "dependencies": [],
            "alternateFileId": 0,
            "isServerPack": false,
            "fileFingerprint": 1492762656,
            "modules": [
                {
                    "name": "META-INF",
                    "fingerprint": 3478797913
                },
                {
                    "name": "mezz",
                    "fingerprint": 3798239254
                },
                {
                    "name": "jei-icon.png",
                    "fingerprint": 2007185424
                },
                {
                    "name": "pack.mcmeta",
                    "fingerprint": 2606738017
                },
                {
                    "name": "assets",
                    "fingerprint": 1792614426
                }
            ]
        },
        {
            "id": 5846878,
            "gameId": 432,
            "modId": 238222,
            "isAvailable": true,
            "displayName": "19.21.0.247 for Fabric 1.21.1",
            "fileName": "jei-1.21.1-fabric-19.21.0.247.jar",
            "releaseType": 2,
            "fileStatus": 4,
            "hashes": [
                {
                    "value": "dc0c17f98fe518b75aa6ba4b4e57957e1d770073",
                    "algo": 1
                },
                {
                    "value": "7712fb0beebc6692c4c1f35e45b8a6f3",
                    "algo": 2
                }
            ],
            "fileDate": "2024-10-26T08:36:41.43Z",
            "fileLength": 1474082,
            "downloadCount": 0,
            "fileSizeOnDisk": 3341309,
            "downloadUrl": "https://edge.forgecdn.net/files/5846/878/jei-1.21.1-fabric-19.21.0.247.jar",
            "gameVersions": [
                "1.21",
                "Fabric",
                "1.21.1"
            ],
            "sortableGameVersions": [
                {
                    "gameVersionName": "1.21",
                    "gameVersionPadded": "0000000001.0000000021",
                    "gameVersion": "1.21",
                    "gameVersionReleaseDate": "2024-06-14T00:00:00Z",
                    "gameVersionTypeId": 77784
                },
                {
                    "gameVersionName": "Fabric",
                    "gameVersionPadded": "0",
                    "gameVersion": "",
                    "gameVersionReleaseDate": "2022-09-01T00:00:00Z",
                    "gameVersionTypeId": 68441
                },
                {
                    "gameVersionName": "1.21.1",
                    "gameVersionPadded": "0000000001.0000000021.0000000001",
                    "gameVersion": "1.21.1",
                    "gameVersionReleaseDate": "2024-08-08T15:17:53.787Z",
                    "gameVersionTypeId": 77784
                }
            ],
            "dependencies": [],
            "alternateFileId": 0,
            "isServerPack": false,
            "fileFingerprint": 2798579048,
            "modules": [
                {
                    "name": "META-INF",
                    "fingerprint": 1108685786
                },
                {
                    "name": "assets",
                    "fingerprint": 2284906622
                },
                {
                    "name": "fabric.mod.json",
                    "fingerprint": 102010748
                },
                {
                    "name": "jei-1.21.1-fabric-refmap.json",
                    "fingerprint": 148796946
                },
                {
                    "name": "jei-icon.png",
                    "fingerprint": 2007185424
                },
                {
                    "name": "jei.accesswidener",
                    "fingerprint": 421432334
                },
                {
                    "name": "jei.mixins.json",
                    "fingerprint": 623332486
                },
                {
                    "name": "mezz",
                    "fingerprint": 2215307582
                },
                {
                    "name": "pack.mcmeta",
                    "fingerprint": 2606738017
                }
            ]
        },
        {
            "id": 5846879,
            "gameId": 432,
            "modId": 238222,
            "isAvailable": true,
            "displayName": "19.21.0.247 for Forge 1.21.1",
            "fileName": "jei-1.21.1-forge-19.21.0.247.jar",
            "releaseType": 2,
            "fileStatus": 4,
            "hashes": [
                {
                    "value": "a8843f9511aa66e1307572f3e632b32597325d26",
                    "algo": 1
                },
                {
                    "value": "c26ba0bdc342db42af91c88b38ec04a3",
                    "algo": 2
                }
            ],
            "fileDate": "2024-10-26T08:36:46.167Z",
            "fileLength": 1442485,
            "downloadCount": 0,
            "fileSizeOnDisk": 3382465,
            "downloadUrl": "https://edge.forgecdn.net/files/5846/879/jei-1.21.1-forge-19.21.0.247.jar",
            "gameVersions": [
                "1.21",
                "Forge",
                "1.21.1"
            ],
            "sortableGameVersions": [
                {
                    "gameVersionName": "1.21",
                    "gameVersionPadded": "0000000001.0000000021",
                    "gameVersion": "1.21",
                    "gameVersionReleaseDate": "2024-06-14T00:00:00Z",
                    "gameVersionTypeId": 77784
                },
                {
                    "gameVersionName": "Forge",
                    "gameVersionPadded": "0",
                    "gameVersion": "",
                    "gameVersionReleaseDate": "2022-10-01T00:00:00Z",
                    "gameVersionTypeId": 68441
                },
                {
                    "gameVersionName": "1.21.1",
                    "gameVersionPadded": "0000000001.0000000021.0000000001",
                    "gameVersion": "1.21.1",
                    "gameVersionReleaseDate": "2024-08-08T15:17:53.787Z",
                    "gameVersionTypeId": 77784
                }
            ],
            "dependencies": [],
            "alternateFileId": 0,
            "isServerPack": false,
            "fileFingerprint": 3647623145,
            "modules": [
                {
                    "name": "META-INF",
                    "fingerprint": 2328668133
                },
                {
                    "name": "jei-icon.png",
                    "fingerprint": 2007185424
                },
                {
                    "name": "pack.mcmeta",
                    "fingerprint": 2606738017
                },
                {
                    "name": "assets",
                    "fingerprint": 2284906622
                },
                {
                    "name": "mezz",
                    "fingerprint": 2692070206
                }
            ]
        },
        {
            "id": 5846880,
            "gameId": 432,
            "modId": 238222,
            "isAvailable": true,
            "displayName": "19.21.0.247 for NeoForge 1.21.1",
            "fileName": "jei-1.21.1-neoforge-19.21.0.247.jar",
            "releaseType": 1,
            "fileStatus": 4,
            "hashes": [
                {
                    "value": "4d9ed6b03c67148ffe7406a84deb168a632e0a60",
                    "algo": 1
                },
                {
                    "value": "6518e8d125958d51540b03faab99c97e",
                    "algo": 2
                }
            ],
            "fileDate": "2024-10-26T08:36:50.697Z",
            "fileLength": 1442319,
            "downloadCount": 2592864,
            "fileSizeOnDisk": 3382709,
            "downloadUrl": "https://edge.forgecdn.net/files/5846/880/jei-1.21.1-neoforge-19.21.0.247.jar",
            "gameVersions": [
                "1.21",
                "NeoForge",
                "1.21.1"
            ],
            "sortableGameVersions": [
                {
                    "gameVersionName": "1.21",
                    "gameVersionPadded": "0000000001.0000000021",
                    "gameVersion": "1.21",
                    "gameVersionReleaseDate": "2024-06-14T00:00:00Z",
                    "gameVersionTypeId": 77784
                },
                {
                    "gameVersionName": "NeoForge",
                    "gameVersionPadded": "0",
                    "gameVersion": "",
                    "gameVersionReleaseDate": "2023-07-25T00:00:00Z",
                    "gameVersionTypeId": 68441
                },
                {
                    "gameVersionName": "1.21.1",
                    "gameVersionPadded": "0000000001.0000000021.0000000001",
                    "gameVersion": "1.21.1",
                    "gameVersionReleaseDate": "2024-08-08T15:17:53.787Z",
                    "gameVersionTypeId": 77784
                }
            ],
            "dependencies": [],
            "alternateFileId": 0,
            "isServerPack": false,
            "fileFingerprint": 3687585417,
            "modules": [
                {
                    "name": "META-INF",
                    "fingerprint": 3950425043
                },
                {
                    "name": "mezz",
                    "fingerprint": 806438967
                },
                {
                    "name": "jei-icon.png",
                    "fingerprint": 2007185424
                },
                {
                    "name": "pack.mcmeta",
                    "fingerprint": 2606738017
                },
                {
                    "name": "assets",
                    "fingerprint": 2284906622
                }
            ]
        },
        {
            "id": 6075246,
            "gameId": 432,
            "modId": 238222,
            "isAvailable": true,
            "displayName": "15.20.0.106 for Fabric 1.20.1",
            "fileName": "jei-1.20.1-fabric-15.20.0.106.jar",
            "releaseType": 1,
            "fileStatus": 4,
            "hashes": [
                {
                    "value": "dc779e3f78374c56cfba4a7e0ad2d480d08cc20a",
                    "algo": 1
                },
                {
                    "value": "f2536f0f14f67847634ae70d3ca79348",
                    "algo": 2
                }
            ],
            "fileDate": "2025-01-11T01:35:46.457Z",
            "fileLength": 1392720,
            "downloadCount": 331242,
            "fileSizeOnDisk": 3027372,
            "downloadUrl": "https://edge.forgecdn.net/files/6075/246/jei-1.20.1-fabric-15.20.0.106.jar",
            "gameVersions": [
                "Fabric",
                "1.20.1"
            ],
            "sortableGameVersions": [
                {
                    "gameVersionName": "Fabric",
                    "gameVersionPadded": "0",
                    "gameVersion": "",
                    "gameVersionReleaseDate": "2022-09-01T00:00:00Z",
                    "gameVersionTypeId": 68441
                },
                {
                    "gameVersionName": "1.20.1",
                    "gameVersionPadded": "0000000001.0000000020.0000000001",
                    "gameVersion": "1.20.1",
                    "gameVersionReleaseDate": "2023-06-12T14:26:38.477Z",
                    "gameVersionTypeId": 75125
                }
            ],
            "dependencies": [],
            "alternateFileId": 0,
            "isServerPack": false,
            "fileFingerprint": 3408198029,
            "modules": [
                {
                    "name": "META-INF",
                    "fingerprint": 3707870562
                },
                {
                    "name": "assets",
                    "fingerprint": 3269841571
                },
                {
                    "name": "fabric.mod.json",
                    "fingerprint": 2442740006
                },
                {
                    "name": "jei-1.20.1-fabric-refmap.json",
                    "fingerprint": 1056944492
                },
                {
                    "name": "jei-icon.png",
                    "fingerprint": 2007185424
                },
                {
                    "name": "jei.accesswidener",
                    "fingerprint": 2262576366
                },
                {
                    "name": "jei.mixins.json",
                    "fingerprint": 1640667786
                },
                {
                    "name": "mezz",
                    "fingerprint": 650850733
                },
                {
                    "name": "pack.mcmeta",
                    "fingerprint": 2606738017
                }
            ]
        },
        {
            "id": 6075247,
            "gameId": 432,
            "modId": 238222,
            "isAvailable": true,
            "displayName": "15.20.0.106 for Forge 1.20.1",
            "fileName": "jei-1.20.1-forge-15.20.0.106.jar",
            "releaseType": 1,
            "fileStatus": 4,
            "hashes": [
                {
                    "value": "77928d15d797663846cba44de8f79e4e4c5fdbb7",
                    "algo": 1
                },
                {
                    "value": "245af433575aef3a47ae73492196018f",
                    "algo": 2
                }
            ],
            "fileDate": "2025-01-11T01:35:54.057Z",
            "fileLength": 1379220,
            "downloadCount": 7712674,
            "fileSizeOnDisk": 3064910,
            "downloadUrl": "https://edge.forgecdn.net/files/6075/247/jei-1.20.1-forge-15.20.0.106.jar",
            "gameVersions": [
                "1.20.1",
                "Forge"
            ],
            "sortableGameVersions": [
                {
                    "gameVersionName": "1.20.1",
                    "gameVersionPadded": "0000000001.0000000020.0000000001",
                    "gameVersion": "1.20.1",
                    "gameVersionReleaseDate": "2023-06-12T14:26:38.477Z",
                    "gameVersionTypeId": 75125
                },
                {
                    "gameVersionName": "Forge",
                    "gameVersionPadded": "0",
                    "gameVersion": "",
                    "gameVersionReleaseDate": "2022-10-01T00:00:00Z",
                    "gameVersionTypeId": 68441
                }
            ],
            "dependencies": [],
            "alternateFileId": 0,
            "isServerPack": false,
            "fileFingerprint": 2995075772,
            "modules": [
                {
                    "name": "META-INF",
                    "fingerprint": 297100257
                },
                {
                    "name": "assets",
                    "fingerprint": 3269841571
                },
                {
                    "name": "jei-icon.png",
                    "fingerprint": 2007185424
                },
                {
                    "name": "mezz",
                    "fingerprint": 2529619195
                },
                {
                    "name": "pack.mcmeta",
                    "fingerprint": 2606738017
                }
            ]
        }
    ],
    "latestFilesIndexes": [
        {
            "gameVersion": "1.20.1",
            "fileId": 6075247,
            "filename": "jei-1.20.1-forge-15.20.0.106.jar",
            "releaseType": 1,
            "gameVersionTypeId": 75125,
            "modLoader": 1
        },
        {
            "gameVersion": "1.20.1",
            "fileId": 6075246,
            "filename": "jei-1.20.1-fabric-15.20.0.106.jar",
            "releaseType": 1,
            "gameVersionTypeId": 75125,
            "modLoader": 4
        },
        {
            "gameVersion": "1.21.1",
            "fileId": 5846880,
            "filename": "jei-1.21.1-neoforge-19.21.0.247.jar",
            "releaseType": 1,
            "gameVersionTypeId": 77784,
            "modLoader": 6
        },
        {
            "gameVersion": "1.21",
            "fileId": 5846880,
            "filename": "jei-1.21.1-neoforge-19.21.0.247.jar",
            "releaseType": 1,
            "gameVersionTypeId": 77784,
            "modLoader": 6
        },
        {
            "gameVersion": "1.21.1",
            "fileId": 5846879,
            "filename": "jei-1.21.1-forge-19.21.0.247.jar",
            "releaseType": 2,
            "gameVersionTypeId": 77784,
            "modLoader": 1
        },
        {
            "gameVersion": "1.21",
            "fileId": 5846879,
            "filename": "jei-1.21.1-forge-19.21.0.247.jar",
            "releaseType": 2,
            "gameVersionTypeId": 77784,
            "modLoader": 1
        },
        {
            "gameVersion": "1.21.1",
            "fileId": 5846878,
            "filename": "jei-1.21.1-fabric-19.21.0.247.jar",
            "releaseType": 2,
            "gameVersionTypeId": 77784,
            "modLoader": 4
        },
        {
            "gameVersion": "1.21",
            "fileId": 5846878,
            "filename": "jei-1.21.1-fabric-19.21.0.247.jar",
            "releaseType": 2,
            "gameVersionTypeId": 77784,
            "modLoader": 4
        },
        {
            "gameVersion": "1.16.5",
            "fileId": 5846870,
            "filename": "jei-1.16.5-7.8.0.1013.jar",
            "releaseType": 2,
            "gameVersionTypeId": 70886,
            "modLoader": 1
        },
        {
            "gameVersion": "1.18.2",
            "fileId": 5846864,
            "filename": "jei-1.18.2-forge-10.2.1.1009.jar",
            "releaseType": 2,
            "gameVersionTypeId": 73250,
            "modLoader": 1
        },
        {
            "gameVersion": "1.18.2",
            "fileId": 5846863,
            "filename": "jei-1.18.2-fabric-10.2.1.1009.jar",
            "releaseType": 2,
            "gameVersionTypeId": 73250,
            "modLoader": 4
        },
        {
            "gameVersion": "1.19.2",
            "fileId": 5846858,
            "filename": "jei-1.19.2-forge-11.8.1.1034.jar",
            "releaseType": 2,
            "gameVersionTypeId": 73407,
            "modLoader": 1
        },
        {
            "gameVersion": "1.19.2",
            "fileId": 5846857,
            "filename": "jei-1.19.2-fabric-11.8.1.1034.jar",
            "releaseType": 2,
            "gameVersionTypeId": 73407,
            "modLoader": 4
        },
        {
            "gameVersion": "1.19.4",
            "fileId": 5846852,
            "filename": "jei-1.19.4-forge-13.1.0.19.jar",
            "releaseType": 2,
            "gameVersionTypeId": 73407,
            "modLoader": 1
        },
        {
            "gameVersion": "1.19.4",
            "fileId": 5846851,
            "filename": "jei-1.19.4-fabric-13.1.0.19.jar",
            "releaseType": 2,
            "gameVersionTypeId": 73407,
            "modLoader": 4
        },
        {
            "gameVersion": "1.20.4",
            "fileId": 5846848,
            "filename": "jei-1.20.4-neoforge-17.3.1.5.jar",
            "releaseType": 2,
            "gameVersionTypeId": 75125,
            "modLoader": 6
        },
        {
            "gameVersion": "1.20.4",
            "fileId": 5846847,
            "filename": "jei-1.20.4-forge-17.3.1.5.jar",
            "releaseType": 2,
            "gameVersionTypeId": 75125,
            "modLoader": 1
        },
        {
            "gameVersion": "1.20.4",
            "fileId": 5846846,
            "filename": "jei-1.20.4-fabric-17.3.1.5.jar",
            "releaseType": 2,
            "gameVersionTypeId": 75125,
            "modLoader": 4
        },
        {
            "gameVersion": "1.20.1",
            "fileId": 5846810,
            "filename": "jei-1.20.1-forge-15.20.0.105.jar",
            "releaseType": 2,
            "gameVersionTypeId": 75125,
            "modLoader": 1
        },
        {
            "gameVersion": "1.20.1",
            "fileId": 5846809,
            "filename": "jei-1.20.1-fabric-15.20.0.105.jar",
            "releaseType": 2,
            "gameVersionTypeId": 75125,
            "modLoader": 4
        },
        {
            "gameVersion": "1.12.2",
            "fileId": 5846804,
            "filename": "jei_1.12.2-4.16.1.1013.jar",
            "releaseType": 2,
            "gameVersionTypeId": 628,
            "modLoader": 1
        },
        {
            "gameVersion": "1.20.6",
            "fileId": 5846787,
            "filename": "jei-1.20.6-neoforge-18.0.0.66.jar",
            "releaseType": 2,
            "gameVersionTypeId": 75125,
            "modLoader": 6
        },
        {
            "gameVersion": "1.20.6",
            "fileId": 5846786,
            "filename": "jei-1.20.6-forge-18.0.0.66.jar",
            "releaseType": 2,
            "gameVersionTypeId": 75125,
            "modLoader": 1
        },
        {
            "gameVersion": "1.20.6",
            "fileId": 5846785,
            "filename": "jei-1.20.6-fabric-18.0.0.66.jar",
            "releaseType": 2,
            "gameVersionTypeId": 75125,
            "modLoader": 4
        },
        {
            "gameVersion": "1.21.1",
            "fileId": 5802637,
            "filename": "jei-1.21.1-neoforge-19.21.0.246.jar",
            "releaseType": 2,
            "gameVersionTypeId": 77784,
            "modLoader": 6
        },
        {
            "gameVersion": "1.21",
            "fileId": 5802637,
            "filename": "jei-1.21.1-neoforge-19.21.0.246.jar",
            "releaseType": 2,
            "gameVersionTypeId": 77784,
            "modLoader": 6
        },
        {
            "gameVersion": "1.21.1",
            "fileId": 5623554,
            "filename": "jei-1.21.1-forge-19.8.4.113.jar",
            "releaseType": 1,
            "gameVersionTypeId": 77784,
            "modLoader": 1
        },
        {
            "gameVersion": "1.21.1",
            "fileId": 5623551,
            "filename": "jei-1.21.1-fabric-19.8.4.113.jar",
            "releaseType": 1,
            "gameVersionTypeId": 77784,
            "modLoader": 4
        },
        {
            "gameVersion": "1.19.2",
            "fileId": 5106178,
            "filename": "jei-1.19.2-forge-11.6.0.1019.jar",
            "releaseType": 1,
            "gameVersionTypeId": 73407,
            "modLoader": 1
        },
        {
            "gameVersion": "1.19.2",
            "fileId": 5106177,
            "filename": "jei-1.19.2-fabric-11.6.0.1019.jar",
            "releaseType": 1,
            "gameVersionTypeId": 73407,
            "modLoader": 4
        },
        {
            "gameVersion": "1.20.2",
            "fileId": 5101374,
            "filename": "jei-1.20.2-forge-16.0.0.2.jar",
            "releaseType": 2,
            "gameVersionTypeId": 75125,
            "modLoader": 1
        },
        {
            "gameVersion": "1.20.2",
            "fileId": 5101373,
            "filename": "jei-1.20.2-fabric-16.0.0.2.jar",
            "releaseType": 2,
            "gameVersionTypeId": 75125,
            "modLoader": 4
        },
        {
            "gameVersion": "1.12.2",
            "fileId": 5060339,
            "filename": "jei_1.12.2-4.16.1.1004.jar",
            "releaseType": 2,
            "gameVersionTypeId": 628
        },
        {
            "gameVersion": "1.18.2",
            "fileId": 4593548,
            "filename": "jei-1.18.2-forge-10.2.1.1005.jar",
            "releaseType": 1,
            "gameVersionTypeId": 73250,
            "modLoader": 1
        },
        {
            "gameVersion": "1.19.4",
            "fileId": 4592504,
            "filename": "jei-1.19.4-forge-13.1.0.15.jar",
            "releaseType": 1,
            "gameVersionTypeId": 73407,
            "modLoader": 1
        },
        {
            "gameVersion": "1.20",
            "fileId": 4581323,
            "filename": "jei-1.20-forge-14.0.0.11.jar",
            "releaseType": 2,
            "gameVersionTypeId": 75125,
            "modLoader": 1
        },
        {
            "gameVersion": "1.20",
            "fileId": 4581322,
            "filename": "jei-1.20-fabric-14.0.0.11.jar",
            "releaseType": 2,
            "gameVersionTypeId": 75125,
            "modLoader": 4
        },
        {
            "gameVersion": "1.19.3",
            "fileId": 4455246,
            "filename": "jei-1.19.3-forge-12.4.0.22.jar",
            "releaseType": 2,
            "gameVersionTypeId": 73407,
            "modLoader": 1
        },
        {
            "gameVersion": "1.19.3",
            "fileId": 4455245,
            "filename": "jei-1.19.3-fabric-12.4.0.22.jar",
            "releaseType": 2,
            "gameVersionTypeId": 73407,
            "modLoader": 4
        },
        {
            "gameVersion": "1.16.5",
            "fileId": 4371666,
            "filename": "jei-1.16.5-7.8.0.1009.jar",
            "releaseType": 1,
            "gameVersionTypeId": 70886,
            "modLoader": 1
        },
        {
            "gameVersion": "1.17.1",
            "fileId": 4351306,
            "filename": "jei-1.17.1-8.3.1.1002.jar",
            "releaseType": 2,
            "gameVersionTypeId": 73242,
            "modLoader": 1
        },
        {
            "gameVersion": "1.19.2",
            "fileId": 4087658,
            "filename": "jei-1.19.2-forge-11.4.0.286.jar",
            "releaseType": 3,
            "gameVersionTypeId": 73407,
            "modLoader": 1
        },
        {
            "gameVersion": "1.19.2",
            "fileId": 4087656,
            "filename": "jei-1.19.2-fabric-11.4.0.286.jar",
            "releaseType": 3,
            "gameVersionTypeId": 73407,
            "modLoader": 4
        },
        {
            "gameVersion": "1.18.1",
            "fileId": 4060769,
            "filename": "jei-1.18.1-9.4.1.276.jar",
            "releaseType": 2,
            "gameVersionTypeId": 73250,
            "modLoader": 1
        },
        {
            "gameVersion": "1.18.2",
            "fileId": 4030311,
            "filename": "jei-1.18.2-forge-10.1.5.272.jar",
            "releaseType": 3,
            "gameVersionTypeId": 73250,
            "modLoader": 1
        },
        {
            "gameVersion": "1.18.2",
            "fileId": 4030310,
            "filename": "jei-1.18.2-fabric-10.1.5.272.jar",
            "releaseType": 3,
            "gameVersionTypeId": 73250,
            "modLoader": 4
        },
        {
            "gameVersion": "1.19.1",
            "fileId": 3922508,
            "filename": "jei-1.19.1-forge-11.2.0.244.jar",
            "releaseType": 3,
            "gameVersionTypeId": 73407,
            "modLoader": 1
        },
        {
            "gameVersion": "1.19.1",
            "fileId": 3922506,
            "filename": "jei-1.19.1-fabric-11.2.0.244.jar",
            "releaseType": 3,
            "gameVersionTypeId": 73407,
            "modLoader": 4
        },
        {
            "gameVersion": "1.19",
            "fileId": 3903068,
            "filename": "jei-1.19-forge-11.1.1.239.jar",
            "releaseType": 3,
            "gameVersionTypeId": 73407,
            "modLoader": 1
        },
        {
            "gameVersion": "1.19",
            "fileId": 3903066,
            "filename": "jei-1.19-fabric-11.1.1.239.jar",
            "releaseType": 3,
            "gameVersionTypeId": 73407,
            "modLoader": 4
        },
        {
            "gameVersion": "1.18.1",
            "fileId": 3723162,
            "filename": "jei-1.18.1-9.4.1.172.jar",
            "releaseType": 1,
            "gameVersionTypeId": 73250,
            "modLoader": 1
        },
        {
            "gameVersion": "1.18",
            "fileId": 3550020,
            "filename": "jei-1.18-9.0.0.40.jar",
            "releaseType": 2,
            "gameVersionTypeId": 73250,
            "modLoader": 1
        },
        {
            "gameVersion": "1.13.2",
            "fileId": 3272039,
            "filename": "jei-1.13.2-5.0.0.31.jar",
            "releaseType": 3,
            "gameVersionTypeId": 55023
        },
        {
            "gameVersion": "1.15.2",
            "fileId": 3272032,
            "filename": "jei-1.15.2-6.0.3.16.jar",
            "releaseType": 3,
            "gameVersionTypeId": 68722
        },
        {
            "gameVersion": "1.16.4",
            "fileId": 3245003,
            "filename": "jei-1.16.4-7.6.1.74.jar",
            "releaseType": 2,
            "gameVersionTypeId": 70886,
            "modLoader": 1
        },
        {
            "gameVersion": "1.16.3",
            "fileId": 3104018,
            "filename": "jei-1.16.3-7.6.0.51.jar",
            "releaseType": 2,
            "gameVersionTypeId": 70886,
            "modLoader": 1
        },
        {
            "gameVersion": "1.16.3",
            "fileId": 3071356,
            "filename": "jei-1.16.3-7.4.0.40.jar",
            "releaseType": 3,
            "gameVersionTypeId": 70886,
            "modLoader": 1
        },
        {
            "gameVersion": "1.16.2",
            "fileId": 3060935,
            "filename": "jei-1.16.2-7.3.2.28.jar",
            "releaseType": 3,
            "gameVersionTypeId": 70886,
            "modLoader": 1
        },
        {
            "gameVersion": "1.12.2",
            "fileId": 3040523,
            "filename": "jei_1.12.2-4.16.1.301.jar",
            "releaseType": 1,
            "gameVersionTypeId": 628
        },
        {
            "gameVersion": "1.14.4",
            "fileId": 3039707,
            "filename": "jei-1.14.4-6.0.1.30.jar",
            "releaseType": 3,
            "gameVersionTypeId": 64806
        },
        {
            "gameVersion": "1.16.1",
            "fileId": 3028697,
            "filename": "jei-1.16.1-7.0.1.10.jar",
            "releaseType": 3,
            "gameVersionTypeId": 70886,
            "modLoader": 1
        },
        {
            "gameVersion": "1.15.1",
            "fileId": 2855456,
            "filename": "jei-1.15.1-6.0.0.1.jar",
            "releaseType": 3,
            "gameVersionTypeId": 68722
        },
        {
            "gameVersion": "1.14.3",
            "fileId": 2738328,
            "filename": "jei-1.14.3-6.0.0.8.jar",
            "releaseType": 3,
            "gameVersionTypeId": 64806
        },
        {
            "gameVersion": "1.14.2",
            "fileId": 2733474,
            "filename": "jei-1.14.2-6.0.0.3.jar",
            "releaseType": 3,
            "gameVersionTypeId": 64806
        },
        {
            "gameVersion": "1.10.2",
            "fileId": 2561516,
            "filename": "jei_1.10.2-3.14.8.422.jar",
            "releaseType": 2,
            "gameVersionTypeId": 572
        },
        {
            "gameVersion": "1.12",
            "fileId": 2485363,
            "filename": "jei_1.12.2-4.7.11.102.jar",
            "releaseType": 2,
            "gameVersionTypeId": 628
        },
        {
            "gameVersion": "1.12.1",
            "fileId": 2485363,
            "filename": "jei_1.12.2-4.7.11.102.jar",
            "releaseType": 2,
            "gameVersionTypeId": 628
        },
        {
            "gameVersion": "1.12",
            "fileId": 2478647,
            "filename": "jei_1.12.1-4.7.8.95.jar",
            "releaseType": 1,
            "gameVersionTypeId": 628
        },
        {
            "gameVersion": "1.12.1",
            "fileId": 2478647,
            "filename": "jei_1.12.1-4.7.8.95.jar",
            "releaseType": 1,
            "gameVersionTypeId": 628
        },
        {
            "gameVersion": "1.11.2",
            "fileId": 2461378,
            "filename": "jei_1.11.2-4.5.1.296.jar",
            "releaseType": 2,
            "gameVersionTypeId": 599
        },
        {
            "gameVersion": "1.11.2",
            "fileId": 2453428,
            "filename": "jei_1.11.2-4.5.0.294.jar",
            "releaseType": 1,
            "gameVersionTypeId": 599
        },
        {
            "gameVersion": "1.12",
            "fileId": 2442204,
            "filename": "jei_1.12-4.7.0.68.jar",
            "releaseType": 3,
            "gameVersionTypeId": 628
        },
        {
            "gameVersion": "1.8.9",
            "fileId": 2431977,
            "filename": "jei_1.8.9-2.28.18.187.jar",
            "releaseType": 1,
            "gameVersionTypeId": 4
        },
        {
            "gameVersion": "1.10.2",
            "fileId": 2428966,
            "filename": "jei_1.10.2-3.14.7.420.jar",
            "releaseType": 1,
            "gameVersionTypeId": 572
        },
        {
            "gameVersion": "1.11",
            "fileId": 2360492,
            "filename": "jei_1.11-4.1.1.208.jar",
            "releaseType": 2,
            "gameVersionTypeId": 599
        },
        {
            "gameVersion": "1.11",
            "fileId": 2350616,
            "filename": "jei_1.11-4.0.4.199.jar",
            "releaseType": 3,
            "gameVersionTypeId": 599
        },
        {
            "gameVersion": "1.9.4",
            "fileId": 2313650,
            "filename": "jei_1.9.4-3.6.8.225.jar",
            "releaseType": 1,
            "gameVersionTypeId": 552
        },
        {
            "gameVersion": "1.10",
            "fileId": 2310912,
            "filename": "jei_1.10-3.7.1.219.jar",
            "releaseType": 1,
            "gameVersionTypeId": 572
        },
        {
            "gameVersion": "1.9.4",
            "fileId": 2306298,
            "filename": "jei_1.9.4-3.6.2.211.jar",
            "releaseType": 2,
            "gameVersionTypeId": 552
        },
        {
            "gameVersion": "1.9",
            "fileId": 2305823,
            "filename": "jei_1.9.4-3.4.4.208.jar",
            "releaseType": 2,
            "gameVersionTypeId": 552
        },
        {
            "gameVersion": "1.9",
            "fileId": 2304545,
            "filename": "jei_1.9.4-3.4.3.207.jar",
            "releaseType": 1,
            "gameVersionTypeId": 552
        },
        {
            "gameVersion": "1.8.9",
            "fileId": 2292565,
            "filename": "jei_1.8.9-2.28.14.182.jar",
            "releaseType": 2,
            "gameVersionTypeId": 4
        },
        {
            "gameVersion": "1.8.8",
            "fileId": 2275072,
            "filename": "jei_1.8.9-2.16.2.78.jar",
            "releaseType": 1,
            "gameVersionTypeId": 4
        },
        {
            "gameVersion": "1.8",
            "fileId": 2273901,
            "filename": "jei_1.8-2.14.0.139.jar",
            "releaseType": 1,
            "gameVersionTypeId": 4
        },
        {
            "gameVersion": "1.8.8",
            "fileId": 2270928,
            "filename": "jei_1.8.8-2.8.3.39.jar",
            "releaseType": 2,
            "gameVersionTypeId": 4
        },
        {
            "gameVersion": "1.8",
            "fileId": 2270927,
            "filename": "jei_1.8-1.8.3.96.jar",
            "releaseType": 2,
            "gameVersionTypeId": 4
        }
    ],
    "latestEarlyAccessFilesIndexes": [],
    "dateCreated": "2015-11-23T22:55:58.84Z",
    "dateModified": "2025-04-08T00:00:19.717Z",
    "dateReleased": "2025-01-11T01:35:54.057Z",
    "allowModDistribution": true,
    "gamePopularityRank": 1,
    "isAvailable": true,
    "thumbsUpCount": 1,
    "featuredProjectTag": 0
}"#;
        //#endregion

        let result = serde_json::from_str(json);
        assert!(result.is_ok());

        let jei: Mod = result.unwrap();
        assert_eq!(jei.id, 238222);
        assert_eq!(jei.game_id, 432);
        assert_eq!(jei.name, "Just Enough Items (JEI)");
        assert_eq!(jei.slug, "jei");
        assert_eq!(jei.primary_category_id, 423);
        assert!(jei.class_id.is_some());
        assert_eq!(jei.class_id.unwrap(), 6);

        let logo = jei.logo;
        assert_eq!(logo.id, 29069);
        assert_eq!(logo.mod_id, 238222);

        if let Some(allow_mod_distribution) = jei.dynamic_fields.get("allowModDistribution") {
            assert!(allow_mod_distribution.is_boolean());
            assert_eq!(allow_mod_distribution.as_bool().unwrap(), true);
        }

        if let Some(is_available) = jei.dynamic_fields.get("isAvailable") {
            assert!(is_available.is_boolean());
            assert_eq!(is_available.as_bool().unwrap(), true);
        }
    }
}
