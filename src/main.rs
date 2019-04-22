extern crate serialize;

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
    println!("Hello, world!");
}
