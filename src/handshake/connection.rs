use tokio::{io::AsyncWriteExt, net::TcpStream};

use super::frame::{ClientOptions, Frame, HandshakeFlags, RequestType};

pub struct Connection {
    conn: TcpStream,
}

impl Connection {
    pub fn new(conn: TcpStream) -> Self {
        Self { conn }
    }

    pub async fn handshake(mut self) -> anyhow::Result<()> {
        let handshake_flags = HandshakeFlags::FIXED_NEWSTYLE | HandshakeFlags::NO_ZEROES;
        self.send_frame(Frame::ServerHandshake(handshake_flags))
            .await?;

        // Read client flags
        loop {
            self.get_frame(RequestType::ClientOptions).await?;
        }

        Ok(())
    }

    pub async fn send_frame(&mut self, frame: Frame) -> anyhow::Result<()> {
        frame.write(&mut self.conn).await?;
        self.conn.flush().await?;

        Ok(())
    }

    pub async fn get_frame(&mut self, expected: RequestType) -> anyhow::Result<()> {
        Frame::read(&mut self.conn, expected).await?;
        Ok(())
    }
}
