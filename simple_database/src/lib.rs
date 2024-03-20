use std::fs;

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
        String::from_utf8_lossy(&fs::read(file_name).unwrap())
            .parse()
            .unwrap()
    }
}


#[test]
fn test_write_read() {
    let db = SimpleStringDb;
    let data = vec![String::from("0"), String::from("1")];
    let data_in = serde_json::to_string(&data).unwrap();
    db.write_data_to_file("foo", &data_in);
    let data_out = db.read_data_from_file("foo");
    let data_out_deserialised: Vec<String> = serde_json::from_str(&data_out).unwrap();
    assert_eq!(data_in, data_out);
    assert_eq!(data, data_out_deserialised);
}
