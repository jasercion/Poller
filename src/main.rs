extern crate iron;
extern crate urlencoded;
extern crate router;
extern crate serde;
#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
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

    Iron::new(router).http("localhost:3000").unwrap();
}

fn handle_request(_request: &mut Request) -> IronResult<Response> {
   
};


