use binread::BinRead;
use serde::Serialize;

use crate::file::SqPackFile;

#[derive(BinRead, Debug, Serialize)]
#[allow(clippy::upper_case_acronyms)]
#[br(import(size: u32))]
pub struct SQPK {
    #[br(assert(inner_size == size, "Inner size mismatch: {} != {}", inner_size, size))]
    inner_size: u32,

    command: SQPKCommand,
}

#[derive(BinRead, Debug, Serialize)]
pub enum SQPKCommand {
    #[br(magic = b"X")]
    PatchInfo {
        status: u8,
        version: u8,

        #[br(pad_before(1))]
        install_size: u64,
    },
    #[br(magic = b"A")]
    AddData(AddDataCmd),
}

#[derive(BinRead, Debug, Serialize)]
pub struct AddDataCmd {
    file: SqPackFile,

    #[br(map(|x: u32| x << 7))]
    block_offset: u32,
    #[br(map(|x: u32| x << 7))]
    block_number: u32,
    #[br(map(|x: u32| x << 7))]
    block_delete_number: u32,

    #[br(count = block_number)]
    block_data: Vec<u8>,
}