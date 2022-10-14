use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::net::ToSocketAddrs;

use crate::consts;
use crate::consts::NbdOpt;
use crate::handshake::connection::Connection;
use crate::handshake::frame::ClientHandshakeFlags;
use crate::handshake::frame::Frame;
use crate::handshake::frame::HandshakeFlags;
use crate::handshake::frame::RequestType;
use anyhow::anyhow;

#[derive(Debug)]
pub enum InteractionResult {
    Abort,
    Continue,
}

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn bind<T: ToSocketAddrs>(addr: T) -> anyhow::Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Server { listener })
    }

    pub async fn start(mut self) -> anyhow::Result<()> {
        loop {
            let (socket, addr) = self.listener.accept().await?;

            println!("Received connection from {addr}");

            // If TCP sockets are used, both the client and server SHOULD disable Nagle's algorithm
            // (that is, use setsockopt to set the TCP_NODELAY option to non-zero)
            // https://github.com/NetworkBlockDevice/nbd/blob/master/doc/proto.md#protocol-phases
            socket.set_nodelay(true)?;
            self.process(socket).await?;
        }

        Ok(())
    }

    async fn process(&mut self, socket: TcpStream) -> anyhow::Result<()> {
        let mut conn = Connection::new(socket);
        self.handshake(&mut conn).await?;

        Ok(())
    }

    pub async fn handshake(&mut self, conn: &mut Connection) -> anyhow::Result<InteractionResult> {
        let handshake_flags = HandshakeFlags::FIXED_NEWSTYLE | HandshakeFlags::NO_ZEROES;
        conn.send_frame(Frame::ServerHandshake(handshake_flags))
            .await?;

        // Read client flags
        loop {
            let frame = conn.get_frame(RequestType::ClientOptions).await?;
            let options = match frame {
                Frame::ClientOptions(options) => Some(options),
                _ => None,
            };

            if options.is_none() {
                return Err(anyhow!("Invalid frame"));
            }

            let options = options.unwrap();
            if !options
                .flags
                .contains(ClientHandshakeFlags::NO_ZEROES | ClientHandshakeFlags::FIXED_NEWSTYLE)
            {
                return Err(anyhow!("Invalid client flags"));
            }

            // Check if we need to abort at this point
            if let Some(opt) = options.client_options.get(0) {
                match opt {
                    NbdOpt::Abort => {
                        conn.send_frame(Frame::ServerAbort).await?;
                        break;
                    }
                    NbdOpt::StructuredReply => {
                        conn.send_frame(Frame::StructuredReply).await?;
                    }
                    _ => {
                        println!("Inform about unsupported option");
                        conn.send_frame(Frame::Unsupported(*opt)).await?;
                    }
                }
            }
        }

        Ok(InteractionResult::Abort)
    }
}
