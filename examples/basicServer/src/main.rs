extern crate  reckless_service;

use reckless_service::server_instance::ServerInstance;


pub fn main(){
    ServerInstance::new("127.0.0.1:8080".to_owned(), "../../webby/".to_owned()).run_server();
}