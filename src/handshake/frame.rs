use bitflags::bitflags;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::consts::{self, NbdOpt, NbdReply};

#[derive(Debug)]
pub enum Frame {
    ServerHandshake(HandshakeFlags),
    ClientOptions(ClientOptions),
    ServerAbort,
    StructuredReply,
    Unsupported(NbdOpt),
    Invalid,
}

#[derive(Debug)]
pub enum RequestType {
    ClientOptions,
}

#[derive(Debug)]
pub struct ClientOptions {
    pub flags: ClientHandshakeFlags,
    pub client_options: Vec<NbdOpt>,
}

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

impl Frame {
    pub async fn write<T: AsyncWriteExt + Unpin>(self, to: &mut T) -> anyhow::Result<()> {
        match self {
            Frame::ServerHandshake(flags) => {
                // https://github.com/NetworkBlockDevice/nbd/blob/master/doc/proto.md#newstyle-negotiation
                // S: 64 bits, 0x4e42444d41474943 (ASCII 'NBDMAGIC') (as in the old style handshake)
                to.write_u64(consts::NBD_INIT_MAGIC).await?;

                // S: 64 bits, 0x49484156454F5054 (ASCII 'IHAVEOPT') (note different magic number)
                to.write_u64(consts::NBD_OPTS_MAGIC).await?;

                // S: 16 bits, handshake flags
                to.write_u16(flags.bits()).await?;
            }
            Frame::ClientOptions(_) => todo!(),
            Frame::ServerAbort => {
                Self::reply(to, NbdOpt::Abort, NbdReply::Ack, &[]).await?;
            }
            Frame::StructuredReply => {
                Self::reply(to, NbdOpt::StructuredReply, NbdReply::Ack, &[]).await?;
            }
            Frame::Unsupported(opt) => {
                Self::reply(to, opt, NbdReply::NbdRepErrUnsup, &[]).await?;
            }
            Frame::Invalid => todo!(),
        }
        Ok(())
    }

    pub async fn read<T: AsyncReadExt + Unpin>(
        from: &mut T,
        expected: RequestType,
    ) -> anyhow::Result<Self> {
        match expected {
            RequestType::ClientOptions => {
                // TODO check options
                let flags = from.read_u32().await?;

                let magic = from.read_u64().await?;
                println!("magic {:#02x}", magic);
                let opts = read_client_options(from).await?;
                println!("got client opt {opts:?}");
                let mut client_options = Vec::new();

                client_options.push(opts);
                // TODO: handle None properly
                let flags = ClientHandshakeFlags::from_bits(flags).unwrap();

                return Ok(Frame::ClientOptions(ClientOptions {
                    flags,
                    client_options,
                }));
            }
        }

        Ok(Self::Invalid)
    }

    pub async fn reply<T: AsyncWriteExt + Unpin>(
        to: &mut T,
        code: NbdOpt,
        reply_type: NbdReply,
        data: &[u8],
    ) -> anyhow::Result<()> {
        println!("Sending {}, with {data:?}", code as u32);
        to.write_u64(consts::NBD_REP_MAGIC).await?;
        to.write_u32(code as u32).await?;
        to.write_u32(reply_type as u32).await?;
        to.write_u32(data.len() as u32).await?;
        to.write_all(data).await?;
        to.flush().await?;

        Ok(())
    }
}

async fn read_client_options<T: AsyncReadExt + Unpin>(from: &mut T) -> anyhow::Result<NbdOpt> {
    // C: 32 bits, option
    let code = from.read_u32().await?;
    println!("got code {code} {:#02x}", code);
    let option = NbdOpt::from(code);
    // C: 32 bits, length of option data (unsigned)
    let len = from.read_u32().await?;

    println!("Got option {option:?} with length {len}");

    Ok(option)
}
