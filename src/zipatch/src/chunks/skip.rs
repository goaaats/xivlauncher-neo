use binread::BinRead;
use serde::Serialize;

// TODO(goat): REMOVE! Pseudo stub-chunk.

#[derive(BinRead, Debug, Serialize)]
#[allow(clippy::upper_case_acronyms)]
#[br(import(size: u32))]
pub struct SKIP {
    #[br(count(size))]
    #[serde(skip_serializing)]
    data: Vec<u8>,
}