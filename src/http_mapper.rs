use std::collections::HashMap;
use http_req::HTTPRequest;


pub struct Mapper{
	mapping: HashMap<String, fn(&mut HTTPRequest, String)>,
}

impl Mapper{
	pub fn new()->Mapper{
		Mapper{
			mapping: HashMap::new(),
		}
	}
	pub fn add_mapping(&mut self, call_sign: String, funct: fn(&mut HTTPRequest, String)){
		self.mapping.insert(call_sign, funct);
	}
}