use nbd_rs::server::Server;

#[tokio::main]
async fn main() {
    let s = Server::bind("[::]:10809").await.unwrap();
    s.start().await.unwrap();
}