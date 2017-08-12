use std::collections::HashMap;
use file_responder::specific_file_finder;
use router::HttpResponse;


#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct HTTPRequest{
	accept: Option<String>,
	accept_charset: Option<String>,
	accept_encoding: Option<String>,
	accept_language: Option<String>,
	accept_date_time: Option<String>, 
	access_ctrl_req_method: Option<String>,
	authorization: Option<String>,
	cache_ctrl: Option<String>,
	connection: Option<String>,
	cookie: Option<String>,
	content_length: Option<String>,
	content_md5: Option<String>,
	content_type: Option<String>, 
	date: Option<String>,
	expect: Option<String>,
	forwarded: Option<String>,
	from: Option<String>,
	host: Option<String>,
	if_match: Option<String>,
	if_mod_since: Option<String>,
	if_none_match: Option<String>,
	if_range: Option<String>,
	if_unmod_since: Option<String>,
	max_fwd: Option<String>,
	origin: Option<String>,
	pragma: Option<String>,
	proxy_auth: Option<String>,
	range: Option<String>, 
	referer: Option<String>,
	te: Option<String>,
	user_agent: Option<String>,
	upgrade: Option<String>,
	via:Option<String>,
	warning: Option<String>,
	method: Option<String>,
	parameters: Option<HashMap<String,String>>,
	file: Option<String>,
	static_loc: String,
}

/*
*The definitions, getters&setters
*/
impl HTTPRequest{
	pub fn new()->HTTPRequest{
		HTTPRequest{
			accept: None,
			accept_charset: None,
			accept_encoding: None,
			accept_language: None,
			accept_date_time: None, 
			access_ctrl_req_method: None,
			authorization: None,
			cache_ctrl: None,
			connection: None,
			cookie: None,
			content_length: None,
			content_md5: None,
			content_type: None, 
			date: None,
			expect: None,
			forwarded: None,
			from: None,
			host: None,
			if_match: None,
			if_mod_since: None,
			if_none_match: None,
			if_range: None,
			if_unmod_since: None,
			max_fwd: None,
			origin: None,
			pragma: None,
			proxy_auth: None,
			range: None, 
			referer: None,
			te: None,
			user_agent: None,
			upgrade: None,
			via: None,
			warning: None,
			method: None,
			parameters: None,
			file: None,
			static_loc: "./webby/".to_owned(),
		}
	}

	pub fn file_responder(&self, file_name: String)->HttpResponse{
		return specific_file_finder(file_name, self.static_loc.clone());
	}

	pub fn set_static_loc(&mut self, static_: String){
		self.static_loc = static_;
	}
	pub fn get_static_loc(self)->String{
		return self.static_loc
	}

	pub fn set_accept(&mut self, msg:String){
		self.accept = Some(msg);
	}
	pub fn set_accept_charset(&mut self, msg:String){
		self.accept_charset = Some(msg);
	}
	pub fn set_accept_encoding(&mut self, msg:String){
		self.accept_encoding = Some(msg);
	}
	pub fn set_accept_language(&mut self, msg:String){
		self.accept_language = Some(msg);
	}
	pub fn set_accept_date_time(&mut self, msg:String){
		self.accept_date_time = Some(msg);
	}
	pub fn set_access_ctrl_req_method(&mut self, msg:String){
		self.access_ctrl_req_method = Some(msg);
	}
	pub fn set_authorization(&mut self, msg:String){
		self.authorization = Some(msg);
	}
	pub fn set_cache_ctrl(&mut self, msg:String){
		self.cache_ctrl = Some(msg);
	}
	pub fn set_connection(&mut self, msg:String){
		self.connection = Some(msg);
	}
	pub fn set_cookie(&mut self, msg:String){
		self.cookie = Some(msg);
	}
	pub fn set_content_length(&mut self, msg:String){
		self.content_length = Some(msg);
	}
	pub fn set_content_md5(&mut self, msg:String){
		self.content_md5 = Some(msg);
	}
	pub fn set_content_type(&mut self, msg:String){
		self.content_type = Some(msg);
	}
	pub fn set_date(&mut self, msg:String){
		self.date = Some(msg);
	}
	pub fn set_expect(&mut self, msg:String){
		self.expect = Some(msg);
	}
	pub fn set_forwarded(&mut self, msg:String){
		self.forwarded=Some(msg);
	}
	pub fn set_from(&mut self, msg:String){
		self.from = Some(msg);
	}
	pub fn set_host(&mut self, msg:String){
		self.host = Some(msg);
	}
	pub fn set_if_match(&mut self, msg:String){
		self.if_match = Some(msg);
	}
	pub fn set_if_mod_since(&mut self, msg:String){
		self.if_mod_since = Some(msg);
	}
	pub fn set_if_none_match(&mut self, msg:String){
		self.if_none_match=Some(msg);
	}
	pub fn set_if_range(&mut self, msg:String){
		self.if_range = Some(msg);
	}
	pub fn set_if_unmod_since(&mut self, msg:String){
		self.if_unmod_since = Some(msg);
	}
	pub fn set_max_fwd(&mut self, msg:String){
		self.max_fwd = Some(msg);
	}
	pub fn set_origin(&mut self, msg:String){
		self.origin = Some(msg);
	}
	pub fn set_pragma(&mut self, msg:String){
		self.pragma = Some(msg);
	}
	pub fn set_proxy_auth(&mut self, msg:String){
		self.proxy_auth = Some(msg);
	}
	pub fn set_range(&mut self, msg:String){
		self.range = Some(msg);
	}
	pub fn set_referrer(&mut self, msg:String){
		self.referer=Some(msg);
	}
	pub fn set_te(&mut self, msg:String){
		self.te = Some(msg);
	}
	pub fn set_user_agent(&mut self, msg:String){
		self.user_agent = Some(msg);
	}
	pub fn set_upgrade(&mut self, msg:String){
		self.upgrade = Some(msg);
	}
	pub fn set_warning(&mut self, msg:String){
		self.warning = Some(msg);
	}
	pub fn set_method(&mut self, msg:String){
		self.method = Some(msg);
	}
	pub fn set_parameters(&mut self, msg:HashMap<String,String>){
		self.parameters = Some(msg);
	}
	pub fn set_file(&mut self, msg:String){
		self.file = Some(msg);
	}
	pub fn get_accept(&self)->Option<String>{
		return self.accept.clone();
	}
	pub fn get_accept_charset(&self)->Option<String>{
		return self.accept_charset.clone();
	}
	pub fn get_accept_encoding(&self)->Option<String>{
		return self.accept_encoding.clone();
	}
	pub fn get_accept_language(&self)->Option<String>{
		return self.accept_language.clone();
	}
	pub fn get_accept_date_time(&self)->Option<String>{
		return self.accept_date_time.clone();
	}
	pub fn get_access_ctrl_req_method(&self)->Option<String>{
		return self.access_ctrl_req_method.clone();
	}
	pub fn get_authorization(&self)->Option<String>{
		return self.authorization.clone();
	}
	pub fn get_cache_ctrl(&self)->Option<String>{
		return self.cache_ctrl.clone();
	}
	pub fn get_connection(&self)->Option<String>{
		return self.connection.clone();
	}
	pub fn get_cookie(&self)->Option<String>{
		return self.cookie.clone();
	}
	pub fn get_content_length(&self)->Option<String>{
		return self.content_length.clone();
	}
	pub fn get_content_md5(&self)->Option<String>{
		return self.content_md5.clone();
	}
	pub fn get_content_type(&self)->Option<String>{
		return self.content_type.clone();
	}
	pub fn get_date(&self)->Option<String>{
		return self.date.clone();
	}
	pub fn get_expect(&self)->Option<String>{
		return self.expect.clone();
	}
	pub fn get_from(&self)->Option<String>{
		return self.from.clone();
	}
	pub fn get_host(&self)->Option<String>{
		return self.host.clone();
	}
	pub fn get_if_match(&self)->Option<String>{
		return self.if_match.clone();
	}
	pub fn get_if_mod_since(&self)->Option<String>{
		return self.if_mod_since.clone();
	}
	pub fn get_if_range(&self)->Option<String>{
		return self.if_range.clone();
	}
	pub fn get_if_unmod_since(&self)->Option<String>{
		return self.if_unmod_since.clone();
	}
	pub fn get_max_fwd(&self)->Option<String>{
		return self.max_fwd.clone();
	}
	pub fn get_origin(&self)->Option<String>{
		return self.origin.clone();
	}
	pub fn get_pragma(&self)->Option<String>{
		return self.pragma.clone();
	}
	pub fn get_proxy_auth(&self)->Option<String>{
		return self.proxy_auth.clone();
	}
	pub fn get_range(&self)->Option<String>{
		return self.range.clone();
	}
	pub fn get_te(&self)->Option<String>{
		return self.te.clone();
	}
	pub fn get_user_agent(&self)->Option<String>{
		return self.user_agent.clone();
	}
	pub fn get_upgrade(&self)->Option<String>{
		return self.upgrade.clone();
	}
	pub fn get_warning(&self)->Option<String>{
		return self.warning.clone();
	}
	pub fn get_method(&self)->Option<String>{
		return self.method.clone();
	}
	pub fn get_parameters(&self)->Option<HashMap<String,String>>{
		return self.parameters.clone();
	}
	pub fn get_file(&self)->Option<String>{
		return self.file.clone();
	}

}

pub fn parse_variables(vars : String)->HashMap<String, String>{
	let var_components = vars.split("&");
	let mut mappy = HashMap::new();
	if var_components.count() > 1{
		let var_components = vars.split("&");
		for i in var_components{
			let tstring = String::from(i);
			let var_comp = tstring.split("=");
			let mut strangs = vec![];
			for k in var_comp{
				println!("variable component {}", k);
				strangs.push(k);
			}
			mappy.insert(String::from(strangs[0]), String::from(strangs[1].trim()));
		}
		for (variable, value) in &mappy{
			println!("{} : {}", variable, value);
		}
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