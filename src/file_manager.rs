use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Read, Write, IoSlice, Bytes};
use std::borrow::BorrowMut;

pub struct FileManager {
   file: File,
   pub file_length: u64,
}

impl FileManager {
   pub fn new(path: String) -> FileManager {
      let mut file = OpenOptions::new()
      .read(true)
       .write(true)
      .create(true)
      .open(path)
      .unwrap();

      let file_length = file.seek(SeekFrom::End(0)).unwrap();
      return FileManager {file, file_length}
   }

   pub fn read_page(&mut self) {
      let mut s = self.file.borrow_mut().bytes();
      
      // TODO: outer loop to parse row size
      // TODO: loop the number of bytes to read and store into row vector
   }
}