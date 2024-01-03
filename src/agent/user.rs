use crate::*;
use std::sync::Arc;
use tokio::{sync::mpsc, task};

enum Command {
    Send(MessageContents),
    Leave,
}

async fn user_agent(id: Id, msg_tx: LobbySend, mut cmd_rx: mpsc::Receiver<Command>) {
    let mut msg_rx = msg_tx.subscribe();
    loop {
        tokio::select! {
            Ok(msg) = msg_rx.recv() => {
                println!("{msg}");
            },
            Some(cmd) = cmd_rx.recv() => {
                match cmd {
                    Command::Send(contents) => {
                        let msg = Message {
                            author: id,
                            contents: Arc::new(contents),
                        };

                        if msg_tx.send(msg).is_err() {
                            break;
                        }
                    },
                    Command::Leave => {
                        break;
                    }
                }
            },
            else => {
                break;
            }
        }
    }
}

pub struct UserAgent {
    join: task::JoinHandle<()>,
    cmd_tx: mpsc::Sender<Command>,
}

impl UserAgent {
    pub fn spawn(id: Id, msg_tx: LobbySend) -> Self {
        let (cmd_tx, cmd_rx) = mpsc::channel(1);
        let join = tokio::spawn(user_agent(id, msg_tx, cmd_rx));
        Self { join, cmd_tx }
    }

    pub async fn send(&self, contents: MessageContents) {
        self.cmd_tx.send(Command::Send(contents)).await.unwrap();
    }

    pub async fn send_text(&self, text: impl Into<String>) {
        self.send(MessageContents::Text(text.into())).await;
    }

    pub async fn leave(self) {
        let _ = self.cmd_tx.send(Command::Leave).await;
        self.join.await.unwrap()
    }
}
