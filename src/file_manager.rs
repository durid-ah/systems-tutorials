use std::fs::{File, OpenOptions};
use std::iter::Peekable;
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

   fn read_row_size_header(iter: &mut Peekable<Bytes<&mut File>>) -> u16{
      let mut size_buffer :[u8; 2] = [0; 2];
      size_buffer[0] = iter.next()
         .expect("Unable to read byte 1 of size header")
         .unwrap();

      size_buffer[1] = iter.next()
         .expect("Unable to read byte 2 of size header")
         .unwrap();

      return u16::from_le_bytes(size_buffer);
   }

   // TODO: Add a seek page by number function
   
   // TODO: Convert the code below to load a single row (load_next_row()) ?
   // TODO: The row might be a None if end of file reached
   // TODO: What will the end of page implications be?

   // Should this only return a row a time and initialize page in pager?
   // TODO: Figure out how to put the parsed rows into an unitialized  page
   // TODO: Refactor to iterate through every row and load its data from the file if it exists
   pub fn load_page(&mut self) {
      // TODO: Seek file to the right page number
      let mut file_iter = self.file.borrow_mut()
         .bytes()
         .peekable();

      // TODO: Create MaybeUninit Page
 
      // TODO: Use init_pages() as inspiration
      // TODO: Change loop to iterate uninit page and read each rows data into a vector
      // TODO: Take into account end of file possibility and setting rows to None
      while !file_iter.peek().is_none() {
         let row_size = FileManager::read_row_size_header(&mut file_iter);
         let row: Vec<u8> = Vec::with_capacity(row_size as usize);
         for i in 0..row_size {
            let file_byte = file_iter.next()
               .expect(
                  format!("Invalid number of row bytes expected {} got {}", row_size, i)
               ).unwrap();

            row.push(file_byte)
         }
      }
   }
}