mod http_req;
mod http_mapper;
mod req_res;
mod stream_message;
mod router;
mod file_responder;
mod server_instance;

use server_instance::ServerInstance;
use file_responder::specific_file_finder;


fn main() {

    let mut si =  ServerInstance::new("127.0.0.1:8080".to_owned());

    fn helloWorld(http_rq: http_req::HTTPRequest)->router::HttpResponse{
            specific_file_finder("index.html".to_owned())
    }

    si.register_get_route("/helloWorld".to_owned(), helloWorld);
    si.run_server()
}
