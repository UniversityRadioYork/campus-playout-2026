#[cfg(unix)]
use std::path::PathBuf;

#[cfg(unix)]
pub mod unix;

#[allow(async_fn_in_trait, reason = "we only use this trait internally")]
pub trait LiquidsoapBackend {
    type Error;

    async fn skip_track(&self) -> Result<(), Self::Error>;
}

#[cfg(unix)]
pub fn unix<P: Into<PathBuf>>(path: P) -> unix::UnixLiquidsoapBackend {
    unix::UnixLiquidsoapBackend::new(path)
}
