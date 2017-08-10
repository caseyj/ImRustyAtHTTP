
mod http_req;
mod http_mapper;
mod req_res;
mod stream_message;
mod router;
mod file_responder;
mod server_instance;

use server_instance::ServerInstance;



fn main() {
    let mut instance = ServerInstance::new("127.0.0.1:8080".to_owned()).run_server();
}
