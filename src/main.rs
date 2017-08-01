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




pub struct HTTP_REQ{
	pub method: String,
	pub file: String,
	pub params : Option<HashMap<String,String>>,
}

impl HTTP_REQ{
	pub fn new(request: String)->HTTP_REQ{

		let req_components = request.split_whitespace();
		let mut split_req = vec![];
		for comp in req_components{
			split_req.push(comp);
		}
		let _method = String::from(split_req[0]);
		let _file = String::from(split_req[1]);

		let mut dot : String = ".".to_owned();
		dot.push_str(split_req[1]);
		match split_req[0].as_ref(){
			"GET"=>{
				let req_query = HTTP_REQ::parse_get_req(_file);
				let mut get_params = None;
				if req_query.len() > 1{
					get_params = Some(HTTP_REQ::parse_variables(
						String::from(req_query[1].clone())
						)
					);
				}
				dot = ".".to_owned();
				dot.push_str(&req_query[0]);
				HTTP_REQ{
					method: _method,
					file: dot,
					params: get_params,
				}
			},
			"POST"=>{
				HTTP_REQ{
					method: _method,
					file: dot,
					params: Some(HTTP_REQ::parse_variables(
						String::from(split_req[split_req.len()-1])
							)
						),
				}	
			},
			_=>{
				HTTP_REQ{
					method: _method,
					file: String::from("./index.html"),
					params: None,	
				}
			}
		}
	}

	pub fn parse_variables(vars : String)->HashMap<String, String>{
		let var_components = vars.split("&");
		let mut mappy = HashMap::new();
		for i in var_components{
			let tstring = String::from(i);
			let var_comp = tstring.split("=");
			let mut strangs = vec![];
			for k in var_comp{
				println!("variable component {}", k);
				strangs.push(k);
			}
			mappy.insert(String::from(strangs[0]), String::from(strangs[1]));
		}
		for (variable, value) in &mappy{
			println!("{} : {}", variable, value);
		}
		return mappy;
	}

	pub fn parse_get_req(query: String)->Vec<String>{
		let splitter = query.split("?");
		let mut split_query = vec![];
		for i in splitter{
			split_query.push(String::from(i));
		}
		return split_query;
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
		    
		    let req = HTTP_REQ::new(String::from(request));
		    match req.method.as_ref() {
		        "GET"=>get_respond(stream, req),
		        "POST"=> post_respond(stream, req),
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
