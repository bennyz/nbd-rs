use bitflags::bitflags;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::consts::{
    self, NBD_OPT_EXPORT_NAME, NBD_OPT_LIST_META_CONTEXT, NBD_OPT_SET_META_CONTEXT,
    NBD_OPT_STRUCTURED_REPLY,
};

#[derive(Debug)]
pub enum Frame {
    ServerHandshake(HandshakeFlags),
    ClientOptions,
    Invalid,
}

#[derive(Debug)]
pub enum RequestType {
    ClientOptions,
}

#[derive(Debug)]
pub enum NbdOpt {
    Export,
    ExportName,
    Abort,
    List,
    StartTls,
    Info,
    Go,
    StructuredReply,
    ListMetaContext,
    SetMetaContext,
    Invalid,
}

impl From<u32> for NbdOpt {
    fn from(code: u32) -> NbdOpt {
        match code {
            NBD_OPT_EXPORT_NAME => Self::ExportName,
            NBD_OPT_ABORT => Self::Abort,
            NBD_OPT_LIST => Self::List,
            NBD_OPT_STARTTLS => Self::StartTls,
            NBD_OPT_INFO => Self::Info,
            NBD_OPT_GO => Self::Go,
            NBD_OPT_STRUCTURED_REPLY => Self::StructuredReply,
            NBD_OPT_LIST_META_CONTEXT => Self::ListMetaContext,
            NBD_OPT_SET_META_CONTEXT => Self::SetMetaContext,
            _ => Self::Invalid,
        }
    }
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
            Frame::ClientOptions => todo!(),
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
                let options = from.read_u32().await?;

                let magic = from.read_u64().await?;
                println!("magic {:#02x}", magic);
                read_client_options(from).await?;
            }
        }

        Ok(Self::Invalid)
    }
}

async fn read_client_options<T: AsyncReadExt + Unpin>(from: &mut T) -> anyhow::Result<NbdOpt> {
    // C: 32 bits, option
    let code = from.read_u32().await?;
    let option = NbdOpt::from(code);
    // C: 32 bits, length of option data (unsigned)
    let len = from.read_u32().await?;

    println!("Got option {option:?} with length {len}");

    match option {
        NbdOpt::Export => todo!(),
        NbdOpt::ExportName => todo!(),
        NbdOpt::Abort => todo!(),
        NbdOpt::List => todo!(),
        NbdOpt::StartTls => todo!(),
        NbdOpt::Info => todo!(),
        NbdOpt::Go => todo!(),
        NbdOpt::StructuredReply => todo!(),
        NbdOpt::ListMetaContext => todo!(),
        NbdOpt::SetMetaContext => todo!(),
        NbdOpt::Invalid => todo!(),
    }

    Ok(option)
}
