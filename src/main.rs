mod http_req;
mod http_mapper;

use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs::File;
use std::thread;
use std::collections::HashMap;

use http_req::HTTPRequest;
use http_mapper::Mapper;




pub struct HTTP_REQ{
	pub method: String,
	pub file: String,
	pub params : Option<HashMap<String,String>>,
}

impl HTTP_REQ{
	pub fn new(request: String)->HTTP_REQ{
		let iter_lines = request.lines();
		for i in iter_lines{
			println!("NL: {}", i);
		}


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

pub fn client_handle(stream: Result<std::net::TcpStream, std::io::Error>){
	match stream {
		Ok(mut stream) => {
		    let mut buffer = [0; 1024];
		    let buff = stream.read(&mut buffer).unwrap();
		    let request = std::str::from_utf8(&buffer).unwrap();
		    println!("{}",  request);

		    
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
#[derive(Copy, RustcDecodable, RustcEncodable,Clone)]
#[allow(dead_code)]
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
}


/**
*Runs the server, opensa tcp channel and then responds to HTTP Requests
*
*
*/
pub fn server(){

	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

	// accept connections and process them paralell
	for stream in listener.incoming() {
		thread::spawn(||{    
		    client_handle(stream);
		});
	}

}








fn main() {
    server()
}
