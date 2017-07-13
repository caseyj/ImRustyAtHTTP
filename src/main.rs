use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs::File;


pub fn respond(mut stream : TcpStream){
	let mut serve = String::new();
	let mut f = File::open("index.html").unwrap();
	f.read_to_string(&mut serve).unwrap();
	let mut writer : String = "HTTP/1.0 200 OK\nContent-type: text/html\n\n\n".to_owned();

	writer.push_str(&serve);
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
	        	println!("{}",  std::str::from_utf8(&buffer).unwrap());
	        	respond(stream);
	        	
	        }
	        Err(e) => { println!("Failure") }
	    }
	}

}








fn main() {
    println!("Hello, world!");
    listen_to_msg()
}
