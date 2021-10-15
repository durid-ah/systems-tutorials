use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};

pub struct Pager {
   pub file: File,
   pub file_length: u64
}

impl Pager {
   pub fn open_pager(file_name: String) -> Option<Pager> {
      let mut _file = OpenOptions::new()
         .read(true)
         .write(true)
         .create(true)
         .open(file_name)
         .unwrap();

      println!("{:?}", _file);
      let _size = _file.seek(SeekFrom::End(0)).unwrap();
      println!("{:?}", _size);

      
   
      Option::None
   }
}

#[cfg(test)]
mod tests {
   use super::*;
   
   #[test]
   fn constructor_test() {
      let pager  = Pager::open_pager(String::from("test_file.txt"));
      assert_eq!(true, true)
   }
}
