extern crate hyper;

use hyper::{Body, Response, Server};
use hyper::service::service_fn_ok;

fn main () {
    // Set address of socket to listen on...
    let addr = ([127, 0, 0, 1], 3000).into();
    
