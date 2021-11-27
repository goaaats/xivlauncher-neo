use config::PlatformId;
use std::path::PathBuf;

pub mod config;
pub mod file;
pub mod chunks;

pub struct ZiPatchExecutor {
    pub platform: PlatformId,
    pub game_path: PathBuf,
    pub ignore_missing: bool,
    pub ingore_mismatch: bool,
}

#[cfg(test)]
#[test]
fn test_parse() {
    use std::{fs::{self, File}, io::BufReader, time::Instant};

    use binread::{BinRead, Endian, ReadOptions};
    use file::ZiPatchFile;

    use crate::chunks::ZiPatchChunkType;

    let mut options = ReadOptions::default();
    options.endian = Endian::Big; // ← this will be ignored

    let file = File::open("D:\\xivpatches\\H2017.06.06.0000.0001a.patch").unwrap();
    let mut buf_reader = BufReader::new(file);

    let started = Instant::now();

    let zipatch = ZiPatchFile::read_options(&mut buf_reader, &options, ()).unwrap();

    println!("Done in: {}", started.elapsed().as_millis());

    let json = serde_json::to_string_pretty(&zipatch).unwrap();
    fs::write("zipatch.json", json).expect("Unable to write json");

    let chunk = &zipatch.chunks[0];

    match &chunk.data {
        ZiPatchChunkType::FHDR(fhdr) => {
            if let Some(v3d) = &fhdr.ver3_data {
                println!("{:#X}", v3d.repo_type);
            }
        },
        ZiPatchChunkType::APLY(aply) => {},
        ZiPatchChunkType::SQPK(sqpk) => todo!(),
        ZiPatchChunkType::EOF(eof) => todo!(),
    }
}
