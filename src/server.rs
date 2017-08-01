use http_req::{HTTPRequest, parse_get_req, parse_variables};
use http_mapper::Mapper;

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