use std::fs::{File, OpenOptions};
use std::iter::Peekable;
use std::io::{Seek, SeekFrom, Read, Write, IoSlice, Bytes};
use std::borrow::BorrowMut;

use super::size_constants::PAGE_SIZE;

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

   fn read_row_size_header(iter: &mut Peekable<Bytes<&mut File>>) -> u16{
      let mut size_buffer :[u8; 2] = [0; 2];
      size_buffer[0] = iter.next()
         .unwrap_or(Ok(0))
         .unwrap_or(0);

      size_buffer[1] = iter.next()
         .unwrap_or(Ok(0))
         .unwrap_or(0);

      return u16::from_le_bytes(size_buffer);
   }

   pub fn seek_to_page(&mut self, page_num: u64) {
      //TODO: Move offset calculater from pager
      let offset_bytes = page_num * PAGE_SIZE;
      let offset = SeekFrom::Start(offset_bytes);
      
      self.file.seek(offset).expect("unable to seek to page");
   }

   pub fn read_row(&mut self) -> Option<Vec<u8>> {
      let mut file_iter = self.file.borrow_mut()
         .bytes()
         .peekable();

      let row_size = FileManager::read_row_size_header(&mut file_iter);
      println!("Reading:Row_Size:{}", row_size);

      if row_size <= 0 {
         return None;
      }

      let mut row: Vec<u8> = Vec::with_capacity(row_size as usize);
      
      for i in 0..row_size {
         let file_byte = file_iter.next()
            .expect(
               format!("FILE CORRUPTION: Invalid number of row bytes expected {} got {}", row_size, i).as_str()
            ).unwrap();

         row.push(file_byte)
      }

      Some(row)
   }

   pub fn write_row(&mut self, row: &Vec<u8>, size: u16) {
      println!("Writing:Row_Size:{}", size);

      let row_slice = IoSlice::new(row.as_slice());
      self.file.write(&size.to_le_bytes()).expect("unable to read row size");
      self.file.write(&row_slice).expect("unable to read row");
   }
}