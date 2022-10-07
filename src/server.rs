use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::net::ToSocketAddrs;
use tokio_util::codec::Framed;
use tokio_util::codec::LinesCodec;
use futures_util::StreamExt;

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
        let mut client = Framed::new(socket, LinesCodec::new());
        while let Some(Ok(line)) = client.next().await {
            println!("{line}")
        }

        Ok(())

    }
}
