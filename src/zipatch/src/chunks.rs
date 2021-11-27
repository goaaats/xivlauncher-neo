use binread::BinRead;
use serde::Serialize;

pub mod aply;
pub mod fhdr;
pub mod sqpk;
pub mod skip;

#[derive(BinRead, Debug, Serialize)]
#[allow(clippy::upper_case_acronyms)]
#[br(import(size: u32))]
pub enum ZiPatchChunkType {
    #[br(magic = b"FHDR")]
    FHDR(fhdr::FHDR),

    #[br(magic = b"APLY")]
    APLY(aply::APLY),

    #[br(magic = b"SQPK")]
    //SQPK(#[br(pad_after(size.into()))] skip::SKIP),
    SQPK(#[br(args(size))] skip::SKIP),

    #[br(magic = b"EOF_")]
    //EOF(#[br(pad_after(size.into()))] skip::SKIP),
    EOF(#[br(args(size))] skip::SKIP),
}
