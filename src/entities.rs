use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Archive {
    pub name: String,
    pub author: String,
    pub description: String,

    pub cover_path: String,
    pub author_avatar_path: String,
    pub comic_count: i64,
    pub volumes_count: i64,
    pub chapter_count: i64,
    pub picture_count: i64,
    pub comics: Vec<Comic>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArchiveInfo {
    pub name: String,
    pub author: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comic {
    pub id: String,
    pub title: String,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub author_id: String,
    pub author: String,
    pub updated_at: i64,
    pub created_at: i64,
    pub description: String,
    pub chinese_team: String,
    pub finished: bool,

    pub idx: i64,
    pub cover_path: String,
    pub author_avatar_path: String,
    pub volumes_count: i64,
    pub chapter_count: i64,
    pub picture_count: i64,
    pub volumes: Vec<Volume>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComicInfo {
    pub id: String,
    pub title: String,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub author_id: String,
    pub author: String,
    pub updated_at: i64,
    pub created_at: i64,
    pub description: String,
    pub chinese_team: String,
    pub finished: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    pub id: String,
    pub title: String,
    pub updated_at: i64,
    pub created_at: i64,

    pub idx: i64,
    pub cover_path: String,
    pub chapter_count: i64,
    pub picture_count: i64,
    pub chapters: Vec<Chapter>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolumeInfo {
    pub id: String,
    pub title: String,
    pub updated_at: i64,
    pub created_at: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chapter {
    pub id: String,
    pub title: String,
    pub updated_at: i64,
    pub created_at: i64,

    pub idx: i64,
    pub cover_path: String,
    pub picture_count: i64,
    pub pictures: Vec<Picture>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChapterInfo {
    pub id: String,
    pub title: String,
    pub updated_at: i64,
    pub created_at: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Picture {
    pub id: String,
    pub title: String,
    pub width: i64,
    pub height: i64,
    pub format: String,

    pub idx: i64,
    pub picture_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PictureInfo {
    pub id: String,
    pub title: String,
    pub width: i64,
    pub height: i64,
    pub format: String,
}
