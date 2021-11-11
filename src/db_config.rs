use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Write, Read};

#[derive(Serialize, Deserialize)]
pub struct DBConfig {
   pub num_rows: u64
}

impl DBConfig {
   // TODO: Add a way to differentiate configs for different files
   pub fn load() -> DBConfig {
      let mut config_str = String::new();

      let _ = OpenOptions::new()
         .read(true)
         .write(true)
         .create(true)
         .open("dbConfig.json")
         .unwrap()
         .read_to_string(&mut config_str);

      println!("{:?}", config_str);
 
      if config_str.len() == 0 {
         return DBConfig{num_rows: 0}
      }

      return serde_json::from_str(config_str.as_str())
         .unwrap();
   }

   pub fn save(config: &DBConfig) {
      let json = serde_json::to_string(&config).unwrap();

      let _ = OpenOptions::new()
         .write(true)
         .open("dbConfig.json")
         .unwrap()
         .write_all(json.as_bytes());
   }
}