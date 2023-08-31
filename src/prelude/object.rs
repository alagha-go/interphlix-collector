use super::*;

/// Generic like Object that can be used to collect the first page offered small metadata of movie sites
#[derive(Debug, Clone, Deserialize)]
pub struct Object {
    pub id: u32,
    #[serde(alias = "title", alias = "original_name", alias = "original_title")]
    pub name: String,
}