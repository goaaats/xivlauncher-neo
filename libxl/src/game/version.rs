use std::{fs::File, io::{BufReader, Read}, path::Path};
use ring::digest::{Context, Digest, SHA1_FOR_LEGACY_USE_ONLY};
use data_encoding::HEXLOWER;

use super::{constants, repository::Repository};

#[derive(Debug)]
pub enum VersionError {
    PathConversion,
    IOError(std::io::Error),
}

pub fn get_patch_gamever_info(game_path: &Path) -> Result<String, VersionError> {
    let ver = Repository::Boot.get_version(game_path).map_err(|e| {VersionError::IOError(e)})?;
    let hash = get_patch_gamever_hash(game_path)?;
    Ok(format!("{}={}", ver, hash))
}

fn get_patch_gamever_hash(game_path: &Path) -> Result<String, VersionError> {
    let mut output = String::new();
    let num_hashes = constants::PATCH_GAMEVER_HASHES.len();

    for n  in 0..num_hashes {
        let bin_name = constants::PATCH_GAMEVER_HASHES[n];
        let hash = get_file_hash(&game_path.join("boot").join(bin_name))?;
        output.push_str(&hash);

        if n != num_hashes - 1 {
            output.push_str(",");
        }
    }

    Ok(output)
}

fn get_file_hash(path: &Path) -> Result<String, VersionError> {
    let file = File::open(&path).map_err(VersionError::IOError)?;
    let file_length = file.metadata().unwrap().len();
    let digest = sha1_digest(BufReader::new(file))?;

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