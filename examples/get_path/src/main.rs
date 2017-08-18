extern crate reckless_service;

use reckless_service::server_instance::ServerInstance;
use reckless_service::router::HttpResponse;
use reckless_service::http_req::HTTPRequest;


fn main() {

    let mut si =  ServerInstance::new("127.0.0.1:8080".to_owned(), "../../webby/".to_owned());

    fn hello_world(http_rq: HTTPRequest)->HttpResponse{
            http_rq.file_responder("index.html".to_owned())
    }

    si.register_get_route("/helloWorld".to_owned(), hello_world);
    si.run_server()
}
