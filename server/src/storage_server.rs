use merkle_tree::{interface::{FetchRequest, FetchResponse, StoreRequest, StoreResponse}, MerkleTree};


/// StorageServer provides data storage and retrieval along with a Merkle proof of data integrity  
/// Requires a Database with basic write/read capability
pub struct StorageServer<D: Database> {
  pub db: D
}

impl<D: Database> StorageServer<D> {
  pub fn new(db: D) -> Self {
    StorageServer {
      db
    }
  }
}

/// Database defines a trait for storage of "files" which can be any serialiseable type and "hashes" which are strings
pub trait Database {
  fn write_files<T: serde::Serialize>(&self, items: &Vec<T>) -> ();
  fn read_files<T: for<'a> serde::Deserialize<'a>>(&self) -> Vec<T>;
  fn write_hashes(&self, items: &Vec<String>) -> ();
  fn read_hashes(&self) -> Vec<String>;
}

impl<D: Database> StorageServer<D> {

  /// Store files and return root of merkle tree they generate 
  pub fn add_files(&self, store_request: &StoreRequest) -> StoreResponse {
    if store_request.files.len() != store_request.hashes.len() {
      panic!("Error: Number of files is not equal to number of hashes")
    }

    self.db.write_files(&store_request.files);
    self.db.write_hashes(&store_request.hashes);

    let merkle_tree: MerkleTree = MerkleTree::build(&store_request.hashes);

    StoreResponse {
      root: merkle_tree.get_root()
    }
  }

  // Return file of given index along with merkle proof of its existence in Merkle tree built with all files
  pub fn fetch_file(&self, fetch_request: &FetchRequest) -> FetchResponse {
    let files: Vec<String> = self.db.read_files();
    let hashes = self.db.read_hashes();
    let merkle_tree = MerkleTree::build(&hashes);
    
    let index = fetch_request.file_index;
    let proof = merkle_tree.prove(index.try_into().unwrap());

    FetchResponse {
      file: files[index].clone(),
      proof
    }
  }
}