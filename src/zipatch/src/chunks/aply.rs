use binread::BinRead;
use serde::Serialize;

#[derive(BinRead, Debug, Serialize)]
#[allow(clippy::upper_case_acronyms)]
pub struct APLY {
    kind: AplyOptionKind,

         #[br(pad_before(4))]
         #[br(map = |x: u32| x != 0)]
         value: bool,
}

#[derive(BinRead, Debug, Serialize)]
#[br(big, repr = u32)]
pub enum AplyOptionKind {
    IgnoreMissing = 1,
    IngoreOldMismatch = 2,
}