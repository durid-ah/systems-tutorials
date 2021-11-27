use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Write, Read};

#[derive(Serialize, Deserialize)]
pub struct DBConfig {
   pub num_rows: u64,
   pub path: PathBuf
}

impl DBConfig {
   pub fn load(path: PathBuf) -> DBConfig {
      let mut config_str = String::new();

      let _ = OpenOptions::new()
         .read(true)
         .write(true)
         .create(true)
         .open(&path)
         .unwrap()
         .read_to_string(&mut config_str);

      if config_str.len() == 0 {
         return DBConfig{num_rows: 0, path}
      }

      return serde_json::from_str(config_str.as_str())
         .unwrap();
   }

   pub fn save(config: &DBConfig) {
      let json = serde_json::to_string(&config).unwrap();

      let _ = OpenOptions::new()
         .write(true)
         .open(&config.path)
         .unwrap()
         .write_all(json.as_bytes());
   }
}