use std::io::*;
use std::net::{TcpListener, TcpStream};



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
	        }
	        Err(e) => { /* connection failed */ }
	    }
	}

}








fn main() {
    println!("Hello, world!");
    listen_to_msg()
}
