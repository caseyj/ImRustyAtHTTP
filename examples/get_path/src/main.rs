extern crate reckless_service;

use reckless_service::server_instance::ServerInstance;
use reckless_service::router::HttpResponse;
use reckless_service::http_req::HTTPRequest;
use reckless_service::file_responder::specific_file_finder;


fn main() {

    let mut si =  ServerInstance::new("127.0.0.1:8080".to_owned());

    fn helloWorld(http_rq: HTTPRequest)->HttpResponse{
            specific_file_finder("index.html".to_owned())
    }

    si.register_get_route("/helloWorld".to_owned(), helloWorld);
    si.run_server()
}
