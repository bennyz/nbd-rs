pub const NBD_DEFAULT_PORT: i32 = 10809;
pub const NBD_FLAG_FIXED_NEWSTYLE: u16 = 1 << 0;
pub const NBD_FLAG_NO_ZEROES: u16 = 1 << 1;

pub const NBD_FLAG_C_FIXED_NEWSTYLE: u32 = 1;
pub const NBD_FLAG_C_NO_ZEROES: u32 = 1 << 1;
pub const NBD_INIT_MAGIC: u64 = 0x4e42444d41474943;
pub const NBD_OPTS_MAGIC: u64 = 0x49484156454F5054;
pub const NBD_REP_MAGIC: u64 = 0x3e889045565a9;
pub const NBD_SIMPLE_REPLY_MAGIC: u32 = 0x67446698;
pub const NBD_REQUEST_MAGIC: u32 = 0x25609513;

pub const NBD_OPT_EXPORT_NAME: u32 = 1;
pub const NBD_OPT_ABORT: u32 = 2;
pub const NBD_OPT_LIST: u32 = 3;
pub const NBD_OPT_STARTTLS: u32 = 5;
pub const NBD_OPT_INFO: u32 = 6;
pub const NBD_OPT_GO: u32 = 7;
pub const NBD_OPT_STRUCTURED_REPLY: u32 = 8;
pub const NBD_OPT_LIST_META_CONTEXT: u32 = 9;
pub const NBD_OPT_SET_META_CONTEXT: u32 = 10;

pub const NBD_REQUEST_SIZE: u32 = 28;
pub const NBD_CMD_FLAG_FUA: u16 = 1 << 0;
pub const NBD_CMD_FLAG_NO_HOLE: u16 = 1 << 1;
pub const NBD_CMD_FLAG_DF: u16 = 1 << 2;
pub const NBD_CMD_FLAG_REQ_ONE: u16 = 1 << 3;
pub const NBD_CMD_FLAG_FAST_ZERO: u16 = 1 << 4;

// Reply errors
pub const NBD_REP_FLAG_ERROR: u32 = 1 << 31;

// Custom
pub const MIN_BLOCK_SIZE: u64 = 1;
pub const PREFERRED_BLOCK_SIZE: u64 = 4096;
pub const MAX_BLOCK_SIZE: u64 = 32 * 1024 * 1024;

// Flags https://github.com/NetworkBlockDevice/nbd/blob/master/doc/proto.md#transmission-flags
pub const NBD_FLAG_HAS_FLAGS: u16 = 1 << 0;
pub const NBD_FLAG_READ_ONLY: u16 = 1 << 1;
pub const NBD_FLAG_SEND_FLUSH: u16 = 1 << 2;
pub const NBD_FLAG_SEND_FUA: u16 = 1 << 3;
pub const NBD_FLAG_ROTATIONAL: u16 = 1 << 4;
pub const NBD_FLAG_SEND_TRIM: u16 = 1 << 5;
pub const NBD_FLAG_SEND_WRITE_ZEROES: u16 = 1 << 6;
pub const NBD_FLAG_SEND_DF: u16 = 1 << 7;
pub const NBD_FLAG_CAN_MULTI_CONN: u16 = 1 << 8;
pub const NBD_FLAG_SEND_RESIZE: u16 = 1 << 9;
pub const NBD_FLAG_SEND_CACHE: u16 = 1 << 10;
pub const NBD_FLAG_SEND_FAST_ZERO: u16 = 1 << 11;

// Structured reply
pub const NBD_STRUCTURED_REPLY_MAGIC: u32 = 0x668e33ef;
pub const NBD_REPLY_FLAG_DONE: u16 = 1 << 0;

// Chunk types
pub const NBD_REPLY_TYPE_NONE: u16 = 0;
pub const NBD_REPLY_TYPE_OFFSET_DATA: u16 = 1;
pub const NBD_REPLY_TYPE_OFFSET_HOLE: u16 = 2;
pub const NBD_REPLY_TYPE_BLOCK_STATUS: u16 = 5;
