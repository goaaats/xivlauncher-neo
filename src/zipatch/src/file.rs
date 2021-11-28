use binread::{BinRead, NullString, until_eof};
use serde::Serialize;

use crate::{chunks::ZiPatchChunkType, config::PlatformId};

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

#[derive(BinRead, Debug, Serialize)]
pub struct SqPackFile {
    main_id: u16,
    sub_id: u16,
    file_id: u32,

    #[br(calc((sub_id >> 8) as u8))]
    ex_id: u8,
}

impl SqPackFile {
    pub fn get_ex_path(&self) -> String {
        format!("sqpack/{}/", SqPackFile::get_ex_folder(self.ex_id))
    }

    pub fn get_ex_folder(ex: u8) -> String {
        match ex {
            0 => String::from("ffxiv"),
            _ => format!("ex{}", ex),
        }
    }

    pub fn get_file_name(&self, platform: PlatformId) -> String {
        format!("{}{:02x}{:04x}.{}", self.get_ex_path(), self.main_id, self.sub_id, platform)
    }

    pub fn get_index_file_name(&self, platform: PlatformId) -> String {
        format!("{}.index{}", self.get_file_name(platform), if self.file_id == 0 { "".to_string() } else { format!("{}", self.file_id) })
    }

    pub fn get_dat_file_name(&self, platform: PlatformId) -> String {
        format!("{}.dat{}", self.get_file_name(platform), self.file_id)
    }
}