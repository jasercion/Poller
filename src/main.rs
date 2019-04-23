extern crate iron;
extern crate urlencoded;
extern crate router;
extern crate serialize;
#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;
use serialize::{PrettyEncoder, json, Decodable};



struct Poll {
    title: String,
    options: &mut Vec<String>,
    multivote: bool,
    delete: bool,
}

struct Ballot {
    username: String,
    selection: uint,
    poll_id: uint,
}


fn main() {
    println!("Starting Slack Polling server on http://localhost:3000...");
    
    let mut router = Router::new();

    router.get("/", handle_request, "root");
    router.post("/", post_response, "post");
}
