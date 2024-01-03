use autogen::{EchoAgent, Id, Lobby, UserAgent};

#[tokio::main]
async fn main() {
    let lobby = Lobby::new(10);

    let _echo = EchoAgent::spawn(Id("echo"), lobby.tx().clone());

    let user = UserAgent::spawn(Id("user"), lobby.tx().clone());

    for line in std::io::stdin().lines() {
        let line = line.unwrap();

        user.send_text(line).await;
    }
}
