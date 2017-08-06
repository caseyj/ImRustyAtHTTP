use http_req::HTTPRequest;
use router::HttpResponse;
use std::fs::File;
use std::io::Read;


pub fn file_finder(request: HTTPRequest)->HttpResponse{
    return specific_file_finder(request.get_file().unwrap());
}

pub fn specific_file_finder(fileName: String)->HttpResponse{
    let mut serve = String::new();
    let dot= ".".to_owned() + &fileName;
    let mut response: HttpResponse;
	match File::open(dot){
		Ok(mut f)=>{
			f.read_to_string(&mut serve).unwrap();
			response = HttpResponse::new(serve, "200 OK".to_owned(), "text/html".to_owned());
		},
		Err(e)=>{
			//I assume 404 for now
			response = HttpResponse::new("".to_owned(), "404 Not Found".to_owned(), "text/html".to_owned());
		}
	}
	return response;
}