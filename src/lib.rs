mod core;
pub use core::*;

mod agent;
pub use agent::*;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::*;

    #[tokio::test]
    async fn all() {
        let lobby = Lobby::new(10);

        let echo = EchoAgent::spawn(Id("echo"), lobby.tx().clone());

        let user = UserAgent::spawn(Id("user"), lobby.tx().clone());

        let mut observer = lobby.tx().subscribe();

        user.send_text("I am the user!").await;

        assert_eq!(
            observer.recv().await.unwrap(),
            Message {
                author: Id("user"),
                contents: Arc::new(MessageContents::Text("I am the user!".to_string())),
            }
        );

        assert_eq!(
            observer.recv().await.unwrap(),
            Message {
                author: Id("echo"),
                contents: Arc::new(MessageContents::Text("I am the user!".to_string())),
            }
        );

        echo.leave().await;
        user.leave().await;
    }
}
