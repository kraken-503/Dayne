use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Files {
    #[serde(default)]
    pub thumbnail: String,
    #[serde(default)]
    pub preview: String,
    #[serde(default)]
    pub original: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Dimensions {
    #[serde(default)]
    pub width: u32,
    #[serde(default)]
    pub height: u32,
    pub aspect_ratio: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Wallpaper {
    pub id: String,
    pub title: String,
    pub author: Option<Author>,
    pub category: String,
    pub tags: Vec<String>,
    #[serde(default)]
    pub files: Option<Files>, // Optional to prevent crashes when missing in manifest.json
    pub dimensions: Option<Dimensions>,
    pub file_size_bytes: Option<u64>,
    pub color_palette: Option<Vec<String>>,
    pub license: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BaseUrls {
    pub thumbnail: String,
    pub preview: String,
    pub original: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CatalogManifest {
    pub version: u32,
    pub updated_at: Option<String>,
    pub base_urls: BaseUrls,
    pub categories: Vec<String>,
    pub wallpapers: Vec<Wallpaper>,
}
