#[macro_use]
extern crate serde_derive;
extern crate serde;

pub mod client;

use std::env;

use client::Client;

static PERSIST_FILES_CMD: &str = "persist-files";
static RETRIEVE_FILE_CMD: &str = "retrieve-file";

fn main() {
    // init
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Please provide server endpoint as first arg and command as second: eg `cargo run -- http://127.0.0.1:8000 persist-files`");
    }
    let client = Client::new(&args[1]);

    // Handle command
    let cmd = &args[2];
    if cmd == PERSIST_FILES_CMD {
        // Store contents of items in files/ directory
        client.store();
    } else if cmd == RETRIEVE_FILE_CMD {
        if args.len() < 4 {
            panic!("Please provide a file index to retrieve: eg cargo run -- http://127.0.0.1:8000 retrieve-files 4")
        }
        let file_index = &args[3];
        client.fetch(file_index.parse::<usize>().unwrap());
    } else {
        panic!("Please pass a valid command: `persist-files` or `retrieve-file $INDEX`")
    }
}
