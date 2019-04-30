extern crate url;

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use url::form_urlencoded;



// # Request Struct
//
// Created to hold useful infomation
// extracted from the incoming TcpListener
// stream.  `command` refers specifically to
// the slash command which invoked the app while
// `text` holds any information included after
// the command input.  The remainder of the
// information is metadata populated by Slack.
//

struct Request {
    command: String,
    text: String,
    response_url: String,
    user_id: String,
}


fn main() {
    let listener = TcpListener::bind("10.0.0.4:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = Vec::new();
    let mut request = Request {
        command: String::new(),
        text: String::new(),
        response_url: String::new(),
        user_id: String::new(),
    };

    stream.read_to_end(&mut buffer).unwrap();

    let post = form_urlencoded::parse(&buffer);
    
    for val in post {
        match val.0.as_ref() {
            "command" => request.command = val.1.to_string(),
            "text" => request.text = val.1.to_string(),
            "response_url" => request.response_url = val.1.to_string(),
            "user_id" => request.user_id = val.1.to_string(),
            _ => println!("No match for {}", val.0),
        };
        
        println!("({:?}, {:?})\n", val.0, val.1);
    }
}
