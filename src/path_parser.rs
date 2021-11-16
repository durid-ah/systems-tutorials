use std::path::{Path, PathBuf};
use std::ffi::OsStr;

pub fn prepare_path(path: &str) /*-> (Path, PathBuf)*/ {
   let mut file_path = Path::new(&path);
   let mut ext_str = String::new();

   if let Some(ext) = file_path.extension() {
      ext_str.push_str(ext.to_str().unwrap());
      ext_str.push_str(".");
   }

   ext_str.push_str("json");

   let json_path = file_path.with_extension(ext_str);
   println!("{:?}", json_path)
   // return (file_path, json_path)
}
