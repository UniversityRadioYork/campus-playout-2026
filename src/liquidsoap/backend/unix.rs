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
    const COMMAND_PART_SEPARATOR: &'static [u8] = b" ";
    const COMMAND_SKIP_TRACK: &'static [u8] = b"playout.skip";
    const COMMAND_REQUEST_TRACK: &'static [u8] = b"track_requests.push";

    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self { path: path.into() }
    }

    async fn create_connection(&self) -> Result<UnixStream, std::io::Error> {
        UnixStream::connect(&self.path).await
    }

    async fn send_command(
        &self,
        connection: &mut UnixStream,
        command_parts: &[&[u8]],
    ) -> Result<(), std::io::Error> {
        // make sure connection is writable
        connection.writable().await?;

        // send command
        for (i, part) in command_parts.iter().enumerate() {
            if i != 0 {
                connection.write_all(Self::COMMAND_PART_SEPARATOR).await?;
            }
            connection.write_all(part).await?;
        }
        connection.write_all(Self::ACTION_SEND_COMMAND).await?;

        // flush connection
        connection.flush().await
    }
}

impl LiquidsoapBackend for UnixLiquidsoapBackend {
    type Error = UnixLiquidsoapBackendError;

    async fn skip_track(&self) -> Result<(), Self::Error> {
        let mut connection = self.create_connection().await?;
        self.send_command(&mut connection, &[Self::COMMAND_SKIP_TRACK])
            .await?;
        Ok(())
    }

    async fn request_track(&self, url: &str) -> Result<(), Self::Error> {
        let mut connection = self.create_connection().await?;
        self.send_command(
            &mut connection,
            &[Self::COMMAND_REQUEST_TRACK, url.as_bytes()],
        )
        .await?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum UnixLiquidsoapBackendError {
    #[error("io error")]
    Io(#[from] std::io::Error),
}
