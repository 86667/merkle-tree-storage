use merkle_tree::{hash, interface::{FetchRequest, FetchResponse, StoreRequest, StoreResponse}, verify};

fn main() {
    let inputs: Vec<String> = vec!(String::from("0000"),String::from("1"),String::from("2"),String::from("3"),String::from("01000"),String::from("11"),String::from("21"));
    let hashes: Vec<String> = inputs.clone().into_iter().map(|x: String| hash(x.as_ref())).collect();

    let input: StoreRequest = StoreRequest {
        files: inputs.clone(),
        hashes: hashes.clone()
    };
    let res_post: StoreResponse = post("store", input);
    println!("res_post: {:?}", res_post);

    let file_index = 6;
    let input_get = FetchRequest {
      file_index
    };
    let res_get: FetchResponse = get("fetch", input_get);
    println!("res_get: {:?}", res_get);

    // Re-hash the returned file to validate integrity
    let file_hash = hash(&res_get.file.as_ref());
    // Feed re-hashed file along with merkle root and proof in to verify
    let valid_proof = verify(&res_post.root, &file_hash, &res_get.proof);
    println!("valid proof: {}", valid_proof);
}

pub fn get<T, V>(path: &str, body: T) -> V 
where
    T: serde::ser::Serialize,
    V: serde::de::DeserializeOwned
{   
    let endpoint = "http://127.0.0.1:8000";
    let client = reqwest::blocking::Client::new();
    let builder = client.get(&format!("{}/{}", endpoint, path));
    let value = builder.json(&body).send().unwrap().text().unwrap();
    serde_json::from_str(value.as_str()).unwrap()
}


pub fn post<T, V>(path: &str, body: T)  -> V
where
    T: serde::ser::Serialize,
    V: serde::de::DeserializeOwned,
{   
    let endpoint = "http://127.0.0.1:8000";

    let client = reqwest::blocking::Client::new();
    let builder = client.post(&format!("{}/{}", endpoint, path));

    let value = builder.json(&body).send().unwrap().text().unwrap();
    serde_json::from_str(value.as_str()).expect(&format!("failed to parse: {}", value.as_str()))
}