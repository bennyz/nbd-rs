use tokio::net::{TcpListener, ToSocketAddrs, TcpStream};

use crate::{handshake::connection::Connection, consts::NbdOpt};

#[derive(Debug)]
// InteractionResult will be used to determine whether to abort or continue into the transmission phase.
// Transmission phase will be entered either by the client selecting the NBD_OPT_GO option or NBD_OPT_EXPORT_NAME
// https://github.com/NetworkBlockDevice/nbd/blob/master/doc/proto.md#termination-of-the-session-during-option-haggling
pub enum InteractionResult {
    Abort,
    Continue,
}

#[derive(Debug)]
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
        conn.start_handshake().await?;
        loop {
            let option_request = conn.haggle().await?;
            let option = option_request.option;
            match option {
                NbdOpt::Export => todo!(),
                NbdOpt::ExportName => todo!(),
                NbdOpt::Abort => {
                    conn.ack(option).await?;
                    break
                }
                NbdOpt::List => todo!(),
                NbdOpt::StartTls => todo!(),
                NbdOpt::Info => todo!(),
                NbdOpt::Go => todo!(),
                NbdOpt::StructuredReply => {
                    conn.ack(option).await?;
                },
                NbdOpt::ListMetaContext => todo!(),
                NbdOpt::SetMetaContext => todo!(),
                NbdOpt::Empty => todo!(),
            }
        }

        Ok(InteractionResult::Abort)
    }
}
