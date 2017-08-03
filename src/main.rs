mod http_req;
mod http_mapper;
mod server;
mod stream_message;

use std::io::Error;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs::File;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;

use stream_message::StreamMessage;
use http_req::{HTTPRequest, parse_get_req, parse_variables};
use server::SERVER;



pub fn client_handle(stream: Result<std::net::TcpStream, std::io::Error>, send_stringx: Sender<StreamMessage>)->bool{
	let mut success = false;
	match stream {
		Ok(mut stream) => {
		    let mut buffer = [0; 1024];
		    let buff = stream.read(&mut buffer).unwrap();
		    let request = std::str::from_utf8(&buffer).unwrap();

			//we got the request so send it to the server thread
			let msg = StreamMessage::new(stream, String::from(request));
			send_stringx.send(msg).unwrap();

			
			return true;
		}
		Err(e) => {
			println!("Failure"); 
			return success;
		}
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
	let (send_stringx, recieve_stringx): (Sender<StreamMessage>, Receiver<StreamMessage>) = mpsc::channel();

	thread::spawn(move || {
		serv.run(recieve_stringx);
	});

	// accept connections and process them paralell
	for stream in listener.incoming() {

		 let send_str = send_stringx.clone();

		thread::spawn(move ||{    
		    client_handle(stream, send_str);
		});	
	}

}








fn main() {
    server()
}
