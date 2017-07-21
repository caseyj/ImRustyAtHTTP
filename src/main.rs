use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs::File;




pub struct HTTP_REQ{
	pub method: String,
	pub file: String,
}

impl HTTP_REQ{
	pub fn new(_method: String, _file: String)->HTTP_REQ{
		let mut dot : String = ".".to_owned();
		dot.push_str(&_file);
		HTTP_REQ{
			method: _method,
			file: dot,
		}
	}
}


/**
*Responds to the HTTP request sent with either a document or 404 error if document not found
*Will be expanded for POST
*
*
*
*/
pub fn get_respond(mut stream : TcpStream, req: HTTP_REQ){
	let mut serve = String::new();
	let mut writer : String;
	match File::open(req.file){
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

pub fn post_respond(mut stream : TcpStream, req: HTTP_REQ){
	println!("POST!");
}

/**
*Runs the server, opensa tcp channel and then responds to HTTP Requests
*
*
*/
pub fn server(){

	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

	// accept connections and process them serially
	for stream in listener.incoming() {
	    match stream {
	        Ok(mut stream) => {
	        	let mut buffer = [0; 512];
	        	let buff = stream.read(&mut buffer).unwrap();
	        	let request = std::str::from_utf8(&buffer).unwrap();
	        	println!("{}",  request);

	        	let req_components = request.split(" ");
	        	let mut split_req = vec![];
	        	for comp in req_components{
	        		split_req.push(comp);
	        	}
	        	let req = HTTP_REQ::new(split_req[0].to_owned(), (split_req[1].to_owned()));
	        	match(req.method.as_ref()){
	        		"GET"=>get_respond(stream, req),
	        		"POST"=> post_respond(stream, req),
	        		_=>println!("oops")
	        	}
	        	
	        }
	        Err(e) => { println!("Failure") }
	    }
	}

}








fn main() {
    server()
}
