mod http_req;
mod http_mapper;
mod server;

use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs::File;
use std::thread;
use std::collections::HashMap;

use http_req::{HTTPRequest, parse_get_req, parse_variables};
use server::SERVER;

/**
*Responds to the HTTP request sent with either a document or 404 error if document not found
*Will be expanded for POST
*
*
*
*/
pub fn get_respond(mut stream : TcpStream, req: HTTPRequest){
	let mut serve = String::new();
	let mut writer : String;
	println!("getting: {}", req.get_file().unwrap());
	let dot= ".".to_owned() + &req.get_file().unwrap();
	//dot += &req.get_file().unwrap();
	match File::open(dot){
		Ok(mut f)=>{
				f.read_to_string(&mut serve).unwrap();
				writer = "HTTP/1.0 200 OK\nContent-type: text/html\n\n\n".to_owned();
				writer.push_str(&serve);
			},
		Err(e)=>{
			//I assume 404 for now
			writer = "HTTP/1.0 404 OK\nContent-type: text/html\n\n\n".to_owned()
		}
	}
	stream.write(writer.as_bytes());
}

pub fn post_respond(mut stream : TcpStream, req: HTTPRequest){
	println!("POST!");
	get_respond(stream, req);
}

pub fn client_handle(stream: Result<std::net::TcpStream, std::io::Error>, serv: SERVER){
	match stream {
		Ok(mut stream) => {
		    let mut buffer = [0; 1024];
		    let buff = stream.read(&mut buffer).unwrap();
		    let request = std::str::from_utf8(&buffer).unwrap();

			let req_ = serv.parse_request(request.to_string());
			println!("{:?}", req_);
		    match req_.get_method().unwrap().as_ref() {
		        "get"=>get_respond(stream, req_),
		        "post"=> post_respond(stream, req_),
		    	_=>println!("oops")
		  	}        	
		}
		Err(e) => { println!("Failure") }
	}
}




/**
*Runs the server, opensa tcp channel and then responds to HTTP Requests
*
*
*/
pub fn server(){

	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	let serv = SERVER::new();
	// accept connections and process them paralell
	for stream in listener.incoming() {
		let tserv = serv.clone();
		thread::spawn(move ||{    
		    client_handle(stream, tserv);
		});
	}

}








fn main() {
    server()
}
