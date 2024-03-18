#[derive(Serialize, Deserialize, Debug)]
pub struct StoreRequest {
    pub files: Vec<String>,
    pub hashes: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StoreResponse {
    pub root: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FetchRequest {
    pub file_index: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FetchResponse {
    pub file: String,
    pub proof: Vec<String>
}
