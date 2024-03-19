use merkle_tree::{interface::{FetchRequest, FetchResponse, StoreRequest, StoreResponse}, MerkleTree};

pub trait Database {
  fn write_files<T>(&self, items: &Vec<T>) -> ();
  fn write_hashes<T>(&self, items: &Vec<T>) -> ();
  // fn read_file<T>(&self, index: usize) -> T;
  fn read_hashes<T>(&self) -> Vec<T>;
}
pub struct StorageServer<D: Database> {
  pub db: D
}


impl<D: Database> StorageServer<D> {
  pub fn add_files(&self, store_request: &StoreRequest) -> StoreResponse {
    println!("add_files()");
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

  pub fn fetch_file(&self, fetch_request: &FetchRequest) -> FetchResponse {
    // let inputs: Vec<String> = vec!(String::from("0"),String::from("1"),String::from("2"),String::from("3"));
    // let hashes: Vec<String> = inputs.clone().into_iter().map(|x: String| hash(x.as_ref())).collect();

    // let file: String = self.db.read_file(fetch_request.file_index);
    let hashes = self.db.read_hashes();

    let merkle_tree = MerkleTree::build(&hashes);

    FetchResponse {
      // file,
      file: String::from("file"),
      proof: merkle_tree.prove(fetch_request.file_index.try_into().unwrap())
    }
  }
}

impl<D: Database> StorageServer<D> {
    pub fn new(db: D) -> Self {
      StorageServer {
        db
      }
    }
}