extern crate url;

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use url::form_urlencoded;



/// # Request Struct
///
/// Created to hold useful infomation
/// extracted from the incoming TcpListener
/// stream.
///
/// `command`: The slash command which invoked the app
/// `text`: holds any information included after the command input
/// `response_url`: url provided by Slack for command response
/// `user_id`: id of the user which invoked the command
///

struct Request {
    command: String,
    text: String,
    response_url: String,
    user_id: String,
}

/// # handle_connection(mut stream)
///
/// This function parses the incoming TcpStream and stores
/// relevant data in a new `Request` struct.  

fn handle_connection(mut stream: TcpStream) -> Request {
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
            _ => (),
        };
    }
    return request;
}

/// # main()
///
/// The `main()` function does the following:
///
/// 1) binds a TcpListener to a specific port
/// 2) listens for incoming connections in a `for` loop
///  
/// Within the loop subordinate functions are called which
/// parse the request into a `Request` struct, handle the
/// requested command, construct a response, and send the
/// response back to the requesting server.
///

fn main() {
    let listener = TcpListener::bind("10.0.0.4:7878").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let request = handle_connection(stream);

        println!("Returned Request Struct:\n command: {}, text: {}, response_url: {}, user_id: {}", request.command, request.text, request.response_url, request.user_id);
    }
}
