use niri_ipc::{Event, Reply, Request, socket::SOCKET_PATH_ENV};

use async_std::{
    channel::Sender,
    io::{self, BufReadExt, BufReader, WriteExt},
    os::unix::net::UnixStream,
};

use gtk::glib;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NiriSocketErrors {
    #[error("Couldn't get niri socket path: {0}")]
    EnvVar(#[from] std::env::VarError),
    #[error("Couldn't connect to niri ipc socket: {0}")]
    Connection(#[from] io::Error),
}

pub struct NiriSocket {
    inner: BufReader<UnixStream>,
}

impl NiriSocket {
    pub async fn niri_path() -> Result<String, std::env::VarError> {
        std::env::var(SOCKET_PATH_ENV)
    }

    pub async fn new() -> Result<Self, NiriSocketErrors> {
        let path = NiriSocket::niri_path().await?;

        let sock = UnixStream::connect(path).await?;

        Ok(Self {
            inner: BufReader::new(sock),
        })
    }

    pub async fn send(&mut self, request: Request) -> io::Result<Reply> {
        let mut buffer = serde_json::to_string(&request)?;
        buffer.push('\n');

        self.inner.get_mut().write_all(buffer.as_bytes()).await?;
        buffer.clear();
        self.inner.read_line(&mut buffer).await?;

        Ok(serde_json::from_str(&buffer)?)
    }

    // Use with a separate task
    pub async fn receive_events(&mut self, sender: Sender<Event>) {
        if let Err(e) = self.inner.get_mut().shutdown(std::net::Shutdown::Write) {
            glib::g_warning!(
                "NiriSocket",
                "Couldn't shutdown write channel of niri socket connection: {}",
                e
            );
        }

        let mut buffer = String::new();
        loop {
            buffer.clear();

            if let Err(e) = self.inner.read_line(&mut buffer).await {
                glib::g_critical!(
                    "NiriSocket",
                    "Failed to read line from the socket ipc: {}",
                    e
                );
                break;
            }

            match serde_json::from_str(&buffer) {
                Ok(event) => {
                    if let Err(e) = sender.send(event).await {
                        glib::g_warning!("NiriSocket", "Failed to send event: {}", e);
                    }
                }
                Err(e) => glib::g_warning!("NiriSocket", "Couldn't parse event: {}", e),
            }
        }
    }
}
