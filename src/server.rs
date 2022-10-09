use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::net::ToSocketAddrs;

use crate::frame::Frame;
use crate::frame::HandshakeFlags;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn bind<T: ToSocketAddrs>(addr: T) -> anyhow::Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Server { listener })
    }

    pub async fn start(self) -> anyhow::Result<()> {
        loop {
            let (socket, addr) = self.listener.accept().await?;

            // If TCP sockets are used, both the client and server SHOULD disable Nagle's algorithm
            // (that is, use setsockopt to set the TCP_NODELAY option to non-zero)
            // https://github.com/NetworkBlockDevice/nbd/blob/master/doc/proto.md#protocol-phases
            socket.set_nodelay(true)?;
            self.process(socket).await?;
        }

        Ok(())
    }

    async fn process(&self, socket: TcpStream) -> anyhow::Result<()> {
        let conn = Connection::new(socket);
        conn.handshake().await?;

        Ok(())
    }
}

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

        Ok(())
    }

    pub async fn send_frame(&mut self, frame: Frame) -> anyhow::Result<()> {
        frame.write(&mut self.conn).await?;
        self.conn.flush().await?;

        Ok(())
    }
}
