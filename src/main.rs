mod http_req;
mod http_mapper;

use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs::File;
use std::thread;
use std::collections::HashMap;

use http_req::{HTTPRequest, parse_get_req, parse_variables};
use http_mapper::Mapper;




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

#[allow(dead_code)]
#[derive(Clone)]
pub struct SERVER{
	http_mapping: Mapper,
}

impl SERVER{
	pub fn new()->SERVER{

		let mut mapping = Mapper::new();
		mapping.add_mapping("accept".to_lowercase(), HTTPRequest::set_accept);
		mapping.add_mapping("connection".to_lowercase(), HTTPRequest::set_connection);
		mapping.add_mapping("accept-charset".to_lowercase(), HTTPRequest::set_accept_charset);
		mapping.add_mapping("accept-encoding".to_lowercase(), HTTPRequest::set_accept_encoding);
		mapping.add_mapping("accept-language".to_lowercase(), HTTPRequest::set_accept_language);
		mapping.add_mapping("accept-Datetime".to_lowercase(), HTTPRequest::set_accept_date_time);
		mapping.add_mapping("Access-Control-Request-Method".to_lowercase(), HTTPRequest::set_access_ctrl_req_method);
		mapping.add_mapping("Access-Control-Request-Headers".to_lowercase(), HTTPRequest::set_access_ctrl_req_method);
		mapping.add_mapping("Authorization".to_lowercase(), HTTPRequest::set_authorization);
		mapping.add_mapping("Cache-Control".to_lowercase(), HTTPRequest::set_cache_ctrl);
		mapping.add_mapping("cookie".to_lowercase(), HTTPRequest::set_cookie);
		mapping.add_mapping("Content-Length".to_lowercase(), HTTPRequest::set_content_length);
		mapping.add_mapping("Content-MD5".to_lowercase(), HTTPRequest::set_content_md5);
		mapping.add_mapping("Content-Type".to_lowercase(), HTTPRequest::set_content_type);
		mapping.add_mapping("Date".to_lowercase(), HTTPRequest::set_date);
		mapping.add_mapping("Expect".to_lowercase(), HTTPRequest::set_expect);
		mapping.add_mapping("Forwarded".to_lowercase(), HTTPRequest::set_forwarded);
		mapping.add_mapping("From".to_lowercase(), HTTPRequest::set_from);
		mapping.add_mapping("Host".to_lowercase(), HTTPRequest::set_host);
		mapping.add_mapping("If-Match".to_lowercase(), HTTPRequest::set_if_match);
		mapping.add_mapping("If-Modified-Since".to_lowercase(), HTTPRequest::set_if_mod_since);
		mapping.add_mapping("If-None-Match".to_lowercase(), HTTPRequest::set_if_none_match);
		mapping.add_mapping("If-Range".to_lowercase(), HTTPRequest::set_if_range);
		mapping.add_mapping("If-Unmodified-Since".to_lowercase(), HTTPRequest::set_if_unmod_since);
		mapping.add_mapping("Max-Forwards".to_lowercase(), HTTPRequest::set_max_fwd);
		mapping.add_mapping("Origin".to_lowercase(), HTTPRequest::set_origin);
		mapping.add_mapping("Pragma".to_lowercase(), HTTPRequest::set_pragma);
		mapping.add_mapping("Proxy-Authorization".to_lowercase(), HTTPRequest::set_proxy_auth);
		mapping.add_mapping("Range".to_lowercase(), HTTPRequest::set_range);
		mapping.add_mapping("Referer".to_lowercase(), HTTPRequest::set_referrer);
		mapping.add_mapping("TE".to_lowercase(), HTTPRequest::set_te);
		mapping.add_mapping("User-Agent".to_lowercase(), HTTPRequest::set_user_agent);
		mapping.add_mapping("Upgrade".to_lowercase(), HTTPRequest::set_upgrade);
		mapping.add_mapping("Via".to_lowercase(), HTTPRequest::set_connection);
		mapping.add_mapping("Warning".to_lowercase(), HTTPRequest::set_warning);
		SERVER{
			http_mapping: mapping
		}
	}
	/*
	*parses a request given to the server into an HTTP Request object
	*/
	pub fn parse_request(self, request: String)->HTTPRequest{
		//lets start with an empty request object
		let mut new_req = HTTPRequest::new();
		//get the lines from the request and iterate over them
		let iter_lines = request.lines();
		for i in iter_lines{
			//split on colon and get the count and reset it into an iterator since it has been consumed
			let mut split_colon = i.split(": ");
			let count_colon_split = split_colon.count();
			let mut split_colon = i.split(": ");
			//match to number of sides of the iterator
			match count_colon_split{
				1=>{
					//this is typically the first or last line depending on the type of request so we figure that out
					let mut first_line_split = i.split(" ");
					if first_line_split.count() > 1{
						//this is the first line and contains info in the following format:
						//METHOD FILE[?PARAMETERS] HTTP VERSION
						let mut first_line_split = i.split(" ");
						//the method is the first thing seen
						new_req.set_method(first_line_split.nth(0).unwrap().to_lowercase());
						//the file is the second
						new_req.set_file(first_line_split.nth(0).unwrap().to_lowercase());
						//if it is a get request we better check it for parameters and split that up
						match new_req.get_method().unwrap().as_ref() {
							"get" =>{
								//parse for get parameters
								let get_split = parse_get_req(new_req.get_file().unwrap());
								//if its greater than 1 in size there are get parameters and we must set those
								if get_split.len() > 1{
									new_req.set_parameters(parse_variables(get_split[1].clone().to_string()));
									new_req.set_file(get_split[0].clone());
								}
							},
							_=>{ 
								println!("No get test") 
							}
						}
					}
					else{
						//we assume these are post parameters and set them as such
						if new_req.get_method().unwrap()!="get".to_string(){
							new_req.set_parameters(parse_variables(i.to_string()));
						}
					}
				},
				2=>{
					println!("{}", i);
					if self.http_mapping.mapping.contains_key(&split_colon.nth(0).unwrap().to_lowercase()){
						let mut split_colon = i.split(": ");
						//println!("key supposed to be: {}", &split_colon.nth(0).unwrap().to_lowercase());
						let fnct = self.http_mapping.mapping[&split_colon.nth(0).unwrap().to_lowercase()].fnct;
						fnct(&mut new_req,split_colon.nth(0).unwrap().to_lowercase());
					}
					else{
						println!("key not found: {}", &split_colon.nth(0).unwrap().to_lowercase())
					}
				},
				3=>{
					println!("Something went wrong in request parsing soooooo");
				}
				_=> println!("Something went wrong in request parsing soooooo")
			}
		}
		return new_req;
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
