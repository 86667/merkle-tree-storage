#[macro_use] 
extern crate rocket;
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use merkle_tree::interface::{FetchRequest, FetchResponse, StoreRequest, StoreResponse};
use rocket::serde::json::Json;


#[post("/store", format = "application/json", data = "<store_request>")]
pub fn store(store_request: Json<StoreRequest>) -> Json<StoreResponse> {
    println!("store_request: {:?}", store_request);
    Json(StoreResponse {
        root: String::from("root")
    })
}

#[get("/fetch", format = "application/json", data = "<fetch_request>")]
pub fn fetch(fetch_request: Json<FetchRequest>) -> Json<FetchResponse> {
    println!("fetch_request: {:?}", fetch_request);
    Json(FetchResponse {
        file: String::from("file"),
        proof: vec!()
    })
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![fetch, store])
}
