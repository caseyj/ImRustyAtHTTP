use std::collections::HashMap;
use std::fmt;
use http_req::HTTPRequest;
/**
*This struct will capture all facets of a response 
*   and eventually have a formatting fuction
*
*/
pub struct HttpResponse{
    pub content: String,
    pub response_code: String,
    pub version: String,
    pub content_type: String,
}
impl HttpResponse{
    pub fn new(content_: String, response: String, content_typed: String)->HttpResponse{
        HttpResponse{
            content : content_,
            response_code: response,
            version: "HTTP/1.1".to_uppercase(),
            content_type: content_typed,
        }
    }
}

impl fmt::Display for HttpResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} \n Content-Type: {} \n\n{}", self.version, self.response_code, self.content_type, self.content)
    }
}

pub struct RouteFunct{
    pub rt_fnct : fn(HTTPRequest)->HttpResponse,
}

impl RouteFunct{
    pub fn new( funct:  fn(HTTPRequest)->HttpResponse)->RouteFunct{
        RouteFunct{
            rt_fnct: funct
        }
    }
}

impl Clone for RouteFunct{
	fn clone(&self)->RouteFunct{
		RouteFunct{
			rt_fnct: self.rt_fnct
		}
	}
}

#[derive(Clone)]
pub struct Router{
    pub rt_funct : HashMap<String, RouteFunct>,
}

impl Router{
    pub fn new()->Router{
        Router{
            rt_funct: HashMap::new(),
        }
    }

    pub fn register_route(&mut self, route: String, funct: fn(HTTPRequest)->HttpResponse){
        self.rt_funct.insert(route, RouteFunct::new(funct));
    }

    pub fn run_route(&self, route: String, request: HTTPRequest)->HttpResponse{
        let fnct = self.rt_funct[&route].rt_fnct;
        return fnct(request);
    }

}

