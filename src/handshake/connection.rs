use anyhow::anyhow;
use bitflags::bitflags;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::consts::{
    self, NbdOpt, NbdReply, NBD_FLAG_FIXED_NEWSTYLE, NBD_FLAG_NO_ZEROES, NBD_OPTS_MAGIC,
    NBD_REP_MAGIC,
};

bitflags! {
    pub struct HandshakeFlags: u16 {
        const FIXED_NEWSTYLE = consts::NBD_FLAG_FIXED_NEWSTYLE;
        const NO_ZEROES      = consts::NBD_FLAG_NO_ZEROES;
    }

    pub struct ClientHandshakeFlags: u32 {
        const FIXED_NEWSTYLE = consts::NBD_FLAG_C_FIXED_NEWSTYLE;
        const NO_ZEROES      = consts::NBD_FLAG_C_NO_ZEROES;
    }
}

#[derive(Debug)]
pub struct ClientOptionRequest {
    pub option: NbdOpt,
    pub data: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct Connection {
    conn: TcpStream,
}

impl Connection {
    pub fn new(conn: TcpStream) -> Self {
        Connection { conn }
    }

    pub async fn start_handshake(&mut self) -> anyhow::Result<()> {
        // Server send init, magic, flags
        self.conn.write_u64(consts::NBD_INIT_MAGIC).await?;
        self.conn.write_u64(consts::NBD_OPTS_MAGIC).await?;
        let handshake_flags = HandshakeFlags::FIXED_NEWSTYLE | HandshakeFlags::NO_ZEROES;
        self.conn.write_u16(handshake_flags.bits()).await?;

        // Now we need to loop and look at the clients options
        let flags = self.conn.read_u32().await?;
        let client_flags = ClientHandshakeFlags::from_bits(flags).unwrap();
        if !client_flags
            .contains(ClientHandshakeFlags::NO_ZEROES | ClientHandshakeFlags::FIXED_NEWSTYLE)
        {
            return Err(anyhow!("Unknown client flags {:#02x}", client_flags));
        }

        Ok(())
    }

    pub async fn haggle(&mut self) -> anyhow::Result<ClientOptionRequest> {
        // Check IHAVEOPT
        let magic = self.conn.read_u64().await?;
        if magic != NBD_OPTS_MAGIC {
            return Err(anyhow!(
                "Bad magic received {:#02x}, expected {:#02x}",
                magic,
                NBD_OPTS_MAGIC
            ));
        }

        let option = self.conn.read_u32().await?;
        let len = self.conn.read_u32().await?;
        let mut client_request = ClientOptionRequest {
            option: NbdOpt::from(option),
            data: None,
        };

        if len > 0 {
            let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);
            self.conn.read_exact(&mut buffer).await?;
            client_request.data = Some(buffer);
        }
        dbg!(&client_request);
        Ok(client_request)
    }

    pub async fn ack(&mut self, option: NbdOpt) -> anyhow::Result<()> {
        self.reply(option, NbdReply::Ack, 0, b"").await?;
        Ok(())
    }

    pub async fn unsupported(&mut self, option: NbdOpt) -> anyhow::Result<()> {
        self.reply(option, NbdReply::NbdRepErrUnsup, 0, b"").await?;
        Ok(())
    }

    pub async fn reply(
        &mut self,
        option: NbdOpt,
        reply_type: NbdReply,
        len: u32,
        data: &[u8],
    ) -> anyhow::Result<()> {
        self.conn.write_u64(NBD_REP_MAGIC).await?;
        self.conn.write_u32(option as u32).await?;
        self.conn.write_u32(reply_type as u32).await?;
        self.conn.write_u32(len).await?;
        self.conn.write_all(data).await?;

        self.conn.flush().await?;

        Ok(())
    }
}
