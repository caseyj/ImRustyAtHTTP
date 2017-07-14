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


pub fn respond(mut stream : TcpStream, req: HTTP_REQ){
	let mut serve = String::new();
	println!("{}", req.file);
	let mut writer : String;
	match File::open(req.file){
		Ok(mut f)=>{
				f.read_to_string(&mut serve).unwrap();
				writer = "HTTP/1.0 200 OK\nContent-type: text/html\n\n\n".to_owned();
				writer.push_str(&serve);
			},
		Err(e)=>{
			writer = "HTTP/1.0 400 OK\nContent-type: text/html\n\n\n".to_owned()
		}
	}
	
	println!("response: {}", writer);
	stream.write(writer.as_bytes());
}




pub fn listen_to_msg(){

	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

	// accept connections and process them serially
	for stream in listener.incoming() {
	    match stream {
	        Ok(mut stream) => {
	        	println!("connected!");
	        	let mut buffer = [0; 512];
	        	let buff = stream.read(&mut buffer).unwrap();
	        	let request = std::str::from_utf8(&buffer).unwrap();
	        	println!("{}",  request);

	        	let req_components = request.split(" ");
	        	let mut split_req = vec![];
	        	for comp in req_components{
	        		println!("comp : {}", comp);
	        		split_req.push(comp);
	        	}
	        	println!("{:?}", split_req);
	        	println!("{} : {} ", split_req[0], split_req[1]);
	        	respond(stream, HTTP_REQ::new(split_req[0].to_owned(), (split_req[1].to_owned())));	
	        }
	        Err(e) => { println!("Failure") }
	    }
	}

}








fn main() {
    println!("Hello, world!");
    listen_to_msg()
}
