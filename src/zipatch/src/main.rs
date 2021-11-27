use std::io::Read;

use memmap2::{Mmap, MmapOptions};

fn main() {
    use std::{
        fs::{self, File},
        io::BufReader,
        time::Instant,
    };

    use binread::{BinRead, Endian, ReadOptions};
    use zipatch::file::ZiPatchFile;

    use zipatch::chunks::ZiPatchChunkType;

    let mut options = ReadOptions::default();
    options.endian = Endian::Big; // ← this will be ignored

    let file = File::open("D:\\xivpatches\\H2017.06.06.0000.0001a.patch").unwrap();
    let mmap = unsafe { Mmap::map(&file).unwrap() };
    let mut cursor = binread::io::Cursor::new(&mmap[..]);

    let started = Instant::now();

    let zipatch = ZiPatchFile::read_options(&mut cursor, &options, ()).unwrap();

    println!("Done in: {}", started.elapsed().as_millis());

    let json = serde_json::to_string_pretty(&zipatch).unwrap();
    fs::write("zipatch.json", json).expect("Unable to write json");

    let chunk = &zipatch.chunks[0];

    match &chunk.data {
        ZiPatchChunkType::FHDR(fhdr) => {
            if let Some(v3d) = &fhdr.ver3_data {
                println!("RepoKind: {:#X}", v3d.repo_type);
            }
        }
        ZiPatchChunkType::APLY(aply) => {}
        ZiPatchChunkType::SQPK(sqpk) => todo!(),
        ZiPatchChunkType::EOF(eof) => todo!(),
    }

    println!("{} chunks processed", zipatch.chunks.len());
}
