use std::path::PathBuf;

#[derive(Clone)]
pub struct JinglesConfig {
    pub main_playlist: PathBuf,
    pub morning_playlist: PathBuf,
    pub afternoon_playlist: PathBuf,
    pub evening_playlist: PathBuf,
    pub promos_playlist_id: String,
}
