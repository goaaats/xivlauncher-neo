use std::{fs::File, io::{BufReader, Read}, path::Path};
use ring::digest::{Context, Digest, SHA1_FOR_LEGACY_USE_ONLY};
use data_encoding::HEXLOWER;

#[derive(Debug)]
pub enum VersionError {
    PathConversion,
    IOError(std::io::Error),
}

pub fn get_boot_hash(game_path: &Path) -> Result<String, VersionError> {
    let path = get_file_hash(&game_path.join("boot").join("ffxivboot.exe"))?;

    Ok(path)
}

fn get_file_hash(path: &Path) -> Result<String, VersionError> {
    let file = File::open(&path).map_err(VersionError::IOError)?;
    let file_length = file.metadata().unwrap().len();

    let reader = BufReader::new(file);
    let digest = sha1_digest(reader)?;

    let hex = HEXLOWER.encode(digest.as_ref());

    let file_name = path.file_name().ok_or(VersionError::PathConversion)?.to_str().ok_or(VersionError::PathConversion)?;
    Ok(format!("{}/{}/{}", file_name, file_length, hex))
}

fn sha1_digest<R: Read>(mut reader: R) -> Result<Digest, VersionError> {
    let mut context = Context::new(&SHA1_FOR_LEGACY_USE_ONLY);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer).map_err(VersionError::IOError)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}