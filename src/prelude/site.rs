use super::*;


type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// A triat that enables types to be a source of information like Movies, Tvshows, and cast.
#[async_trait]
pub trait Site {
    /// set the main domain for the site the default domain is the movie database domain `themoviedb.org`
    const DOMAIN: & 'static str = "api.themoviedb.org";
    /// it should be an enum that contains the types of content that can be collected from this site. like Movies Peaople and Tvshows.
    type ItemTypes;

    /// collect all the ids from the site. An example is downloding the file exports from the moviedb site.
    async fn collect_ids() -> Result<Vec<Object>>;

    /// collect a particular item by id.
    async fn get<T>(id: u32, item_type: Self::ItemTypes) -> Result<T>;
}