extern crate url;
extern crate reqwest;

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use url::form_urlencoded;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;

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

//#[derive(Serialize, Deserialize)]
//struct Response {
//    text: String,
//}

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

fn response_constructor(Request: request) -> Response {
    let request_text = request.text.split(',').collect();

    let response = Response {
        text: "This is a test string",
    };

    return response;
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
    let client = reqwest::Client::new();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let confirmation = "HTTP/1,1 200 OK\n\n\r\n";
                
                match stream.write(confirmation) {
                    Ok(_) => println!("HTTP 200 Confirmation sent!"),
                    Err(e) => println!("Failed sending confirmation: {}", e),
                }
                
                // handle_connection returns type `Request`
                let request = handle_connection(stream);
                let response_target = request.response_url.as_str();
                
                let mut map = HashMap::new();
                map.insert("text:", "Test Response!");
                
                match client.post(response_target).json(&map).send() {
                    Ok(_) => println!("Message Response Sent to {}!", response_target),
                    Err(e) => println!("Failed sending message: {}", e),
                }
                
                println!("Returned Request Struct:\n command: {}, text: {}, response_url: {}, user_id: {}", request.command, request.text, request.response_url, request.user_id);
            },
            Err(e) => println!("Connection Failed!: {}", e),
        }
    }
}
