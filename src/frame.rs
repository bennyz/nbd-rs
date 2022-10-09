use bitflags::bitflags;
use tokio::io::AsyncWriteExt;

use crate::consts;

pub enum Frame {
    ServerHandshake(HandshakeFlags),
}

bitflags! {
    pub struct HandshakeFlags: u16 {
        const FIXED_NEWSTYLE = consts::NBD_FLAG_FIXED_NEWSTYLE;
        const NO_ZEROES      = consts::NBD_FLAG_NO_ZEROES;
    }
}

impl Frame {
    pub async fn write<T: AsyncWriteExt + Unpin>(self, to: &mut T) -> anyhow::Result<()> {
        match self {
            Frame::ServerHandshake(flags) => {
                // https://github.com/NetworkBlockDevice/nbd/blob/master/doc/proto.md#newstyle-negotiation
                // S: 64 bits, 0x4e42444d41474943 (ASCII 'NBDMAGIC') (as in the old style handshake)
                to.write_u64(consts::NBD_INIT_MAGIC).await?;

                //  S: 64 bits, 0x49484156454F5054 (ASCII 'IHAVEOPT') (note different magic number)
                to.write_u64(consts::NBD_OPTS_MAGIC).await?;

                // S: 16 bits, handshake flags
                to.write_u16(flags.bits()).await?;
            }
        }
        Ok(())
    }
}
