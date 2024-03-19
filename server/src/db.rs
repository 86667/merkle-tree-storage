use std::fs;

use crate::storage_server::Database;

static DB_FILES_FILE_NAME: &str = "files.db";
static DB_HASHES_FILE_NAME: &str = "hashes.db";

/// A simple lcoal filesystem storage mechanism:  
/// - Stores a single vector of some Serialisable "file" type in local filesystem
/// - Stores a single vector of strings which are the hashes of the stored "files"
pub struct SimpleStringDb;

impl SimpleStringDb {
  pub fn new() -> Self {
    SimpleStringDb 
  }

  pub fn write_data_to_file(&self, file_name: &str, data: &str) {
    fs::write(file_name, data).unwrap();
  }

  pub fn read_data_from_file(&self, file_name: &str) -> String {
    String::from_utf8_lossy(&fs::read(file_name).unwrap()).parse().unwrap()
  }
}

impl Database for SimpleStringDb {
  fn write_files<T: serde::Serialize>(&self, items: &Vec<T>) -> () {
    let serialised_data = serde_json::to_string(items).unwrap();
    self.write_data_to_file(DB_FILES_FILE_NAME, &serialised_data)
  }

  fn read_files<T: for<'a> serde::Deserialize<'a>>(&self) -> Vec<T> {
    let data = self.read_data_from_file(DB_FILES_FILE_NAME);
    serde_json::from_str(&data).unwrap()
  }

  fn write_hashes(&self, items: &Vec<String>) {
    let serialised_data = serde_json::to_string(items).unwrap();
    self.write_data_to_file(DB_HASHES_FILE_NAME, &serialised_data)
  }

  fn read_hashes(&self) -> Vec<String> {
    let data = self.read_data_from_file(DB_HASHES_FILE_NAME);
    serde_json::from_str(&data).unwrap()
  }
}


#[test]
fn test_write_read() {
  let db = SimpleStringDb;
  let data = vec!(String::from("0"), String::from("1"));
  let data_in = serde_json::to_string(&data).unwrap();
  db.write_data_to_file("foo", &data_in);
  let data_out = db.read_data_from_file("foo");
  let data_out_deserialised: Vec<String> = serde_json::from_str(&data_out).unwrap();
  assert_eq!(data_in, data_out);
  assert_eq!(data, data_out_deserialised);
}