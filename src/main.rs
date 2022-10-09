use nbd_rs::server::Server;

#[tokio::main]
async fn main() {
    let s = Server::bind("[::]:1111").await.unwrap();
    s.start().await.unwrap();
}
