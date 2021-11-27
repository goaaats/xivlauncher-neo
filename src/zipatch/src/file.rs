use binread::{BinRead, NullString, until_eof};
use serde::Serialize;

use crate::chunks::ZiPatchChunkType;

#[derive(BinRead, Debug, Serialize)]
#[br(magic = b"\x91ZIPATCH\x0D\x0A\x1A\x0A")]
pub struct ZiPatchFile {
    #[br(parse_with = until_eof)]
    //#[br(count = 15)]
    pub chunks: Vec<ZiPatchChunk>,
}

#[derive(BinRead, Debug, Serialize)]
// TODO(goat): Assert checksum validity
pub struct ZiPatchChunk {
    size: u32,

    // TODO(goat): Can we store the offset into the file here, to checksum later?
    // [#br(calc())]
    // offset: i64,
    // #[br(restore_position)]
    // #[br(count = size)]
    // #[serde(skip_serializing)]
    // raw_data: Vec<u8>,

    #[br(args(size))]
    pub data: ZiPatchChunkType,

    checksum: u32,
}