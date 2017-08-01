use std::collections::HashMap;
use http_req::HTTPRequest;

pub struct FunctWrap{
	pub fnct: fn(&mut HTTPRequest, String)
}
impl FunctWrap{
	pub fn new(function: fn(&mut HTTPRequest, String))->FunctWrap{
		FunctWrap{
			fnct: function
		}
	}
}
impl Clone for FunctWrap{
	fn clone(&self)->FunctWrap{
		FunctWrap{
			fnct: self.fnct
		}
	}
}

#[derive(Clone)]
pub struct Mapper{
	pub mapping: HashMap<String, FunctWrap>,
}

impl Mapper{
	pub fn new()->Mapper{
		Mapper{
			mapping: HashMap::new(),
		}
	}
	pub fn add_mapping(&mut self, call_sign: String, funct: fn(&mut HTTPRequest, String)){
		self.mapping.insert(call_sign, FunctWrap::new(funct));
	}
}