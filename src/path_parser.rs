use std::path::{Path, PathBuf};

pub fn prepare_path(path: &str) -> (PathBuf, PathBuf) {
   let file_path = Path::new(path);
   let mut ext_str = String::new();

   if let Some(ext) = file_path.extension() {
      ext_str.push_str(ext.to_str().unwrap());
      ext_str.push_str(".");
   }

   ext_str.push_str("json");

   let json_path = file_path.with_extension(ext_str);
   return (file_path.to_owned(), json_path); 
}
