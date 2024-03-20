#[macro_use]
extern crate rocket;

extern crate server;
use merkle_tree::interface::{FetchRequest, FetchResponse, StoreRequest, StoreResponse};
use rocket::{serde::json::Json, State};
use server::storage_server::StorageServer;
use simple_database::SimpleStringDb;

#[post("/store", format = "application/json", data = "<store_request>")]
pub fn store(
    server: &State<StorageServer<SimpleStringDb>>,
    store_request: Json<StoreRequest>,
) -> Json<StoreResponse> {
    Json(server.add_files(&store_request))
}

#[get("/fetch", format = "application/json", data = "<fetch_request>")]
pub fn fetch(
    server: &State<StorageServer<SimpleStringDb>>,
    fetch_request: Json<FetchRequest>,
) -> Json<FetchResponse> {
    Json(server.fetch_file(&fetch_request))
}

#[launch]
fn rocket() -> _ {
    let server = StorageServer::new(SimpleStringDb::new());
    rocket::build()
        .mount("/", routes![fetch, store])
        .manage(server)
}
