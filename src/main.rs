mod http_req;
mod http_mapper;
mod req_res;
mod stream_message;
mod router;
mod file_responder;

use std::io::prelude::*;
use std::net::{TcpListener};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;


use stream_message::StreamMessage;
use http_req::{HTTPRequest};
use req_res::{ReqResEngine};
use router::HttpResponse;
use file_responder::{specific_file_finder};


pub fn slash(req: HTTPRequest)->HttpResponse{
	println!("Default mapping of the {} route to index.html", req.get_file().unwrap());
	return specific_file_finder("/index.html".to_owned());
}

pub fn client_handle(stream: Result<std::net::TcpStream, std::io::Error>, send_stringx: Sender<StreamMessage>){
	match stream {
		Ok(mut stream) => {
		    let mut buffer = [0; 1024];
		    stream.read(&mut buffer).unwrap();
		    let request = std::str::from_utf8(&buffer).unwrap();

			//we got the request so send it to the server thread
			let msg = StreamMessage::new(stream, String::from(request));
			send_stringx.send(msg).unwrap();

			
	
		}
		Err(e) => {
			println!("Failure {}", e); 
	
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
	let mut serv = ReqResEngine::new();
	serv.register_get_route("/".to_owned() , slash);
	serv.register_post_route("/".to_owned() , slash);
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
