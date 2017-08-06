mod http_req;
mod http_mapper;
mod server;
mod stream_message;
mod router;
mod file_responder;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;
use std::fs::File;


use stream_message::StreamMessage;
use http_req::{HTTPRequest, parse_get_req, parse_variables};
use server::{SERVER};
use router::HttpResponse;

pub fn slash(req: HTTPRequest)->HttpResponse{
	let mut serve = String::new();
	let mut writer : String;
	let dot= ".".to_owned() + &req.get_file().unwrap() + "index.html";
	let mut response: HttpResponse;
	match File::open(dot){
		Ok(mut f)=>{
			f.read_to_string(&mut serve).unwrap();
			response = HttpResponse::new(serve, "200 OK".to_owned(), "text/html".to_owned());
		},
		Err(e)=>{
			//I assume 404 for now
			writer = "HTTP/1.0 404 OK\nContent-type: text/html\n\n\n".to_owned();
			response = HttpResponse::new("".to_owned(), "404 Not Found".to_owned(), "text/html".to_owned());
		}
	}

	return response

}

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
	let mut serv = SERVER::new();
	serv.register_get_route("/".to_owned() , slash);
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
