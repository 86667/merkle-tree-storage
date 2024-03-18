use merkle_tree::interface::{FetchResponse, FetchRequest, StoreRequest, StoreResponse};

fn main() {
    let input_get = FetchRequest {
        file_index: 0
    };
    let res_get: FetchResponse = get("fetch", input_get);
    println!("res_get: {:?}", res_get);

    let input: StoreRequest = StoreRequest {
        files: vec!(),
        hashes: vec!()
    };
    let res_post: StoreResponse = post("store", input);
    println!("res_post: {:?}", res_post);
}

pub fn get<T, V>(path: &str, body: T) -> V 
where
    T: serde::ser::Serialize,
    V: serde::de::DeserializeOwned
{   
    let endpoint = "http://127.0.0.1:8000";
    let client = reqwest::blocking::Client::new();
    let builder = client.get(&format!("{}/{}", endpoint, path));

    // catch reqwest errors
    let value = match builder.json(&body).send() {
        Ok(v) => v.text().unwrap(),
        Err(e) => panic!("{:?}", e),
    };
    println!("GET return value: {:?}", value);

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

    // catch reqwest errors
    let value = match builder.json(&body).send() {
        Ok(v) => {
            //Reject responses that are too long
            // TODO: set reasonable limit?
            match v.content_length() {
                Some(l) => {
                    if l > 1000000 {
                        println!("POST value ignored because of size: {}", l);
                        panic!("POST value ignored because of size: {}",l);
                    }
                }
                None => (),
            };

            let text = v.text().unwrap();

            if text.contains(&String::from("Error: ")) {
                panic!("Error in post: {}", text);
            }
            text
        }

        Err(e) => panic!("{:?}", e),
    };

    println!("Post return value: {:?}", value);
    serde_json::from_str(value.as_str()).expect(&format!("failed to parse: {}", value.as_str()))
}