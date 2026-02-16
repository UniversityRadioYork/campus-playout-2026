use std::path::PathBuf;

use thiserror::Error;
use tokio::{io::AsyncWriteExt, net::UnixStream};

use crate::liquidsoap::backend::LiquidsoapBackend;

#[derive(Debug, Clone)]
pub struct UnixLiquidsoapBackend {
    path: PathBuf,
}

impl UnixLiquidsoapBackend {
    const ACTION_SEND_COMMAND: &'static [u8] = b"\n";
    const COMMAND_SKIP_TRACK: &'static [u8] = b"srt.skip";

    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self { path: path.into() }
    }

    async fn create_connection(&self) -> Result<UnixStream, std::io::Error> {
        UnixStream::connect(&self.path).await
    }

    async fn send_command(
        &self,
        connection: &mut UnixStream,
        command: &[u8],
    ) -> Result<(), std::io::Error> {
        // make sure connection is writable
        connection.writable().await?;

        // send command
        connection.write(command).await?;
        connection.write(Self::ACTION_SEND_COMMAND).await?;

        // flush connection
        connection.flush().await
    }
}

impl LiquidsoapBackend for UnixLiquidsoapBackend {
    type Error = UnixLiquidsoapBackendError;

    async fn skip_track(&self) -> Result<(), Self::Error> {
        let mut connection = self.create_connection().await?;
        self.send_command(&mut connection, Self::COMMAND_SKIP_TRACK)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum UnixLiquidsoapBackendError {
    #[error("io error")]
    Io(#[from] std::io::Error),
}
