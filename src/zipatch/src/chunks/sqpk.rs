use binread::BinRead;
use serde::Serialize;

#[derive(BinRead, Debug, Serialize)]
#[allow(clippy::upper_case_acronyms)]
pub struct SQPK {
    
}