use crate::*;
use tokio::{sync::mpsc, task};

enum Command {
    Leave,
}

async fn echo_agent(id: Id, msg_tx: LobbySend, mut cmd_rx: mpsc::Receiver<Command>) {
    let mut msg_rx = msg_tx.subscribe();

    loop {
        tokio::select! {
            Ok(msg) = msg_rx.recv() => {
                if msg.author == id {
                    // Do not echo our own messages.
                    continue;
                }

                if msg_tx.send(Message {
                    author: id,
                    contents: msg.contents,
                }).is_err() {
                    // No more receivers.
                    break;
                }
            }
            Some(cmd) = cmd_rx.recv() => {
                match cmd {
                    Command::Leave => {
                        break;
                    },
                }
            }
        }
    }
}

pub struct EchoAgent {
    join: task::JoinHandle<()>,
    cmd_tx: mpsc::Sender<Command>,
}

impl EchoAgent {
    pub fn spawn(id: Id, send: LobbySend) -> Self {
        let (cmd_tx, cmd_rx) = mpsc::channel(1);
        let join = tokio::spawn(echo_agent(id, send, cmd_rx));
        Self { join, cmd_tx }
    }

    pub async fn leave(self) {
        let _ = self.cmd_tx.send(Command::Leave).await;
        self.join.await.unwrap()
    }
}
