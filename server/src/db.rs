use crate::storage_server::Database;


pub struct SimpleStringDb;

impl Database for SimpleStringDb {
  fn write_files<T>(&self, items: &Vec<T>) -> () {

  }

  fn write_hashes<T>(&self, items: &Vec<T>) -> () {
    
  }

  // fn read_file<T>(&self, index: usize) -> T {

  // }

  fn read_hashes<T>(&self) -> Vec<T> {
    return vec!()
  }
}

impl SimpleStringDb {
  pub fn new() -> Self {
    SimpleStringDb 
  }
}