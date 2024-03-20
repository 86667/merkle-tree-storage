use crate::storage_server::Database;

use simple_database::SimpleStringDb;

static DB_FILES_FILE_NAME: &str = "files.db";
static DB_HASHES_FILE_NAME: &str = "hashes.db";


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