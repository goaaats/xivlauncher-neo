#[derive(Debug)]
pub struct PatchList {
  pub patches: Vec<PatchEntry>,
}

#[derive(Debug)]
pub struct PatchEntry {
  pub length: u64,
  pub version_id: String,

  pub hash_type: String,
  pub hash_block_size: u32,
  pub hashes: Option<Vec<String>>,

  pub url: String,
}

impl std::ops::Deref for PatchList {
  type Target = Vec<PatchEntry>;
  fn deref(&self) -> &Vec<PatchEntry> {
      &self.patches
  }
}

impl std::ops::DerefMut for PatchList {
  fn deref_mut(&mut self) -> &mut Vec<PatchEntry> {
      &mut self.patches
  }
}

impl From<String> for PatchList {
  fn from(patch_list: String) -> Self {
    // The first 4 lines are form data and headers, for whatever reason
    let mut lines = patch_list.lines().skip(5).peekable();
    let mut patches: Vec<PatchEntry> = Vec::new();

    while let Some(line) = lines.next() {
      let patch = PatchEntry::from(line);
      patches.push(patch);

      // Skip the last line, it's a form boundary
      if lines.peek().unwrap().starts_with("--") {
        break;
      }
    }

    PatchList { patches }
  }
}

impl From<&str> for PatchList {
  fn from(patch_list: &str) -> Self {
    PatchList::from(String::from(patch_list))
  }
}

impl From<String> for PatchEntry {
  fn from(entry_line: String) -> Self {
    let parts: Vec<&str> = entry_line.split('\t').collect();
    let is_game = parts.len() == 9;

    PatchEntry {
      length: parts[0].parse().unwrap(),
      version_id: parts[4].to_string(),
      hash_type: parts[5].to_string(),
      hash_block_size: if is_game { parts[6].parse().unwrap() } else { 0 },
      hashes: if is_game {
        Some(parts[7].split(',').map(|s| s.to_string()).collect())
      } else {
        None
      },
      url: if is_game {
        parts[8].to_string()
      } else {
        parts[5].to_string()
      },
    }
  }
}

impl From<&str> for PatchEntry {
  fn from(entry_line: &str) -> Self {
    PatchEntry::from(String::from(entry_line))
  }
}
