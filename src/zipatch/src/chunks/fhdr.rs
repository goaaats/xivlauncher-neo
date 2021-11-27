use binread::BinRead;
use serde::Serialize;

#[derive(BinRead, Debug, Serialize)]
#[allow(clippy::upper_case_acronyms)]
pub struct FHDR {
    #[br(little)]
    #[br(map = |x: u32| (x >> 16) as u8)]
    #[br(assert(version == 2 || version == 3, "Unsupported version: {}", version))]
    version: u8,

    #[br(map = |x: u32| String::from_utf8(x.to_be_bytes().to_vec()).unwrap())]
    patch_type: String,

    entry_files: u32,

    #[br(if(version == 3))]
    #[br(pad_after(if version == 3 { 0xB8 } else { 0x08 }))]
    pub ver3_data: Option<FhdrVer3Data>,
}

#[derive(BinRead, Debug, Serialize)]
pub struct FhdrVer3Data {
    add_dirs: u32,
    del_dirs: u32,
    del_data_size: u64,

    minor_version: u32,
    pub repo_type: u32,
    num_commands: u32,

    num_sqpk_add: u32,
    num_sqpk_del: u32,
    num_sqpk_expand: u32,
    num_sqpk_header: u32,
    num_sqpk_file: u32,
}
