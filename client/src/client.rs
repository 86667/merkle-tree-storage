use merkle_tree::{hash, interface::{FetchRequest, FetchResponse, StoreRequest, StoreResponse}, verify};
use simple_database::SimpleStringDb;
use std::fs;

static FILES_DIR_NAME: &str = "files";
static ROOT_STORAGE_FILE_NAME: &str = "root.db";

pub struct Client {
    server_end_point: String,
}

impl Client {
    pub fn new(server_end_point: &String) -> Self {
        Client {
            server_end_point: server_end_point.clone(),
        }
    }

    fn get<T, V>(&self, path: &str, body: &T) -> V
    where
        T: serde::ser::Serialize,
        V: serde::de::DeserializeOwned,
    {
        let client = reqwest::blocking::Client::new();
        let builder = client.get(&format!("{}/{}", self.server_end_point, path));
        let value = builder.json(body).send().unwrap().text().unwrap();
        serde_json::from_str(value.as_str()).unwrap()
    }

    fn post<T, V>(&self, path: &str, body: &T) -> V
    where
        T: serde::ser::Serialize,
        V: serde::de::DeserializeOwned,
    {
        let client = reqwest::blocking::Client::new();
        let builder = client.post(&format!("{}/{}", self.server_end_point, path));

        let value = builder.json(body).send().unwrap().text().unwrap();
        serde_json::from_str(value.as_str()).expect(&format!("failed to parse: {}", value.as_str()))
    }
    
    fn read_files(&self) -> Vec<String> {
      let paths = match fs::read_dir(format!("./{}", FILES_DIR_NAME)) {
        Ok(paths) => paths,
        Err(_) => panic!("client/files/ directory does no exist")
      };

      let paths_count = paths.count();
      let mut inputs: Vec<String> = Vec::with_capacity(paths_count);
      for index in 0..paths_count {
        let path_string: String = String::from(format!("./{}/file", FILES_DIR_NAME)) + &index.to_string();
        let file_contents = match fs::read(&path_string) {
        Ok(content) => content,
          Err(_) => panic!("File {} expected but does not exist. Ensure all files follow the format of file0, file1, file2, etc.", path_string)
        };
        inputs.push(String::from_utf8_lossy(&file_contents)
            .parse()
            .unwrap());
      }
      inputs
    }

    fn pad_files(&self, files: &mut Vec<String>) {
      let num_files = files.len();
      let num_files_to_pad = num_files.next_power_of_two() - num_files;
      files.append(&mut vec![String::from("0");num_files_to_pad]);
    }

    // Read and send files to server
    pub fn store(&self) {
      println!("Sending all files in files/ directory to server for storage.");
      
      let mut files = self.read_files();
      let num_files = files.len();

      // Pad files to be vector of length which equals a power of two
      self.pad_files(&mut files);

      // Hash all files to send along with the files themselves
      let hashes: Vec<String> = files
          .clone()
          .into_iter()
          .map(|x: String| hash(x.as_ref()))
          .collect();

      let input: StoreRequest = StoreRequest {
          files: files.clone(),
          hashes: hashes.clone(),
      };
      let response: StoreResponse = self.post("store", &input);

      // Persist root hash and number of files 
      println!("Writing Merlke root hash to local storage.");
      let client_storage_data = ClientStoredData {
        root_hash: response.root,
        num_files
      };
      SimpleStringDb::new().write_data_to_file(ROOT_STORAGE_FILE_NAME, &build_client_storage_data(&client_storage_data));
      println!("Done.");
    }

    pub fn fetch(&self, file_index: usize) {
      println!("Fetching file from server.");
      let response: FetchResponse = self.get("fetch", &FetchRequest { file_index });

      println!("Verifying file and Merkle proof against local root hash record.");
      // Retreive root hash and number of files stored from local storage
      let client_storage_data: ClientStoredData = parse_client_storage_data(&SimpleStringDb::new().read_data_from_file(ROOT_STORAGE_FILE_NAME));
      if file_index > client_storage_data.num_files - 1 {
        panic!("Cannot fetch file with index {}. Only {} files stored. Files are 0-indexed.", file_index, client_storage_data.num_files);
      }

      self.verify(&response, &client_storage_data.root_hash);
    }

    pub fn verify(&self, fetch_response: &FetchResponse, root_hash: &String) {
      // Re-hash the returned file to validate integrity
      let file_hash = hash(&fetch_response.file.as_ref());
      // Feed re-hashed file along with merkle root and proof in to verify
      let valid_proof = verify(root_hash, &file_hash, &fetch_response.proof);
      if !valid_proof {
        panic!("File succesfully retrieved but proof failed - the file may have been tampered with!")
      }
      println!("Successfully retreived file with index and verified Merkle proof. File contents: {}", fetch_response.file);
    }
}

#[derive(Serialize, Deserialize)]
pub struct ClientStoredData {
  root_hash: String,
  num_files: usize
}

// Vec of root hash and total number of stored files
fn build_client_storage_data(client_stored_data: &ClientStoredData) -> String {
  serde_json::to_string(client_stored_data).unwrap()
}

fn parse_client_storage_data(data: &str) -> ClientStoredData {
  serde_json::from_str(data).unwrap()
}

#[test]
fn test_file_padding() {
  let client = Client {
    server_end_point: String::from("")
  };
  // Should pad to length 8
  let mut files = vec!["0".to_string();6];
  client.pad_files(&mut files);
  assert_eq!(files.len(), 8);

  // Should not pad
  files = vec!["0".to_string();8];
  client.pad_files(&mut files);
  assert_eq!(files.len(), 8);

  // Should pad to length 16
  files = vec!["0".to_string();9];
  client.pad_files(&mut files);
  assert_eq!(files.len(), 16);
}