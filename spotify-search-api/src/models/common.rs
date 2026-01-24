use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct APIResponse {
    pub created: String,
    pub count: u32,
    pub offset: u32,
    pub recordings: Vec<Recording>,
}

#[derive(Debug, Deserialize)]
pub struct Recording {
    pub id: String,

    #[serde(rename = "artist-credit-id")]
    pub artist_credit_id: Option<String>,

    pub title: String,

    pub length: Option<u32>,
    pub video: Option<bool>,

    #[serde(rename = "first-release-date")]
    pub first_release_date: Option<String>,

    #[serde(rename = "artist-credit")]
    pub artist_credit: Option<Vec<ArtistCredit>>,

    pub releases: Option<Vec<Release>>,
}

#[derive(Debug, Deserialize)]
pub struct ArtistCredit {
    pub name: String,
    pub artist: Artist,
}

#[derive(Debug, Deserialize)]
pub struct Artist {
    pub id: String,
    pub name: String,

    #[serde(rename = "sort-name")]
    pub sort_name: String,

    pub disambiguation: Option<String>,
    pub aliases: Option<Vec<Alias>>,
}

#[derive(Debug, Deserialize)]
pub struct Alias {
    pub name: String,

    #[serde(rename = "sort-name")]
    pub sort_name: String,

    pub locale: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Release {
    pub id: String,
    pub title: String,
    pub status: Option<String>,
    pub date: Option<String>,
    pub country: Option<String>,

    #[serde(rename = "track-count")]
    pub track_count: Option<u32>,

    pub media: Option<Vec<Media>>,
}

#[derive(Debug, Deserialize)]
pub struct Media {
    pub format: Option<String>,
    pub position: Option<u32>,

    #[serde(rename = "track-count")]
    pub track_count: Option<u32>,

    pub track: Option<Vec<Track>>,
}

#[derive(Debug, Deserialize)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub length: Option<u32>,
    pub number: Option<String>,
}

