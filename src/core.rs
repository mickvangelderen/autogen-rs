use std::{fmt, sync::Arc};
use tokio::sync::broadcast;

// TODO: Use a GUID or something instead. &'static str is nice for demo.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Id(pub &'static str);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Message {
    pub author: Id,
    pub contents: Arc<MessageContents>,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}: {}", self.author.0, self.contents))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum MessageContents {
    Text(String),
    Attachment(Vec<u8>), // NOTE: May need to further refine the type.
}

impl fmt::Display for MessageContents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(text) => text.fmt(f),
            Self::Attachment(value) => {
                f.write_fmt(format_args!("<attachment of {} bytes>", value.len()))
            }
        }
    }
}

// TODO: We might want to abstract away the implementation details instead through a newtype around
// the channel.
pub type LobbySend = broadcast::Sender<Message>;
pub type LobbyRecv = broadcast::Receiver<Message>;

pub struct Lobby {
    tx: LobbySend,
    _rx: LobbyRecv, // Keep around to ensure sender isn't closed.
}

impl Lobby {
    pub fn new(capacity: usize) -> Lobby {
        let (tx, _rx) = broadcast::channel(capacity);
        Lobby { tx, _rx }
    }

    pub fn tx(&self) -> &LobbySend {
        &self.tx
    }
}
