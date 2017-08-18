extern crate reckless_service;

use std::env;
use reckless_service::server_instance::ServerInstance;

pub fn main(){
    let mut args = env::args();
    let arg_len = args.count();
    args = env::args();

    let mut host = "127.0.0.1:8080".to_owned();
    let mut static_content = "../../webby/".to_owned();

    let mut it_count = 0;

    while it_count < arg_len{
        let arge = args.nth(0).unwrap();
        it_count+=1;
        match arge.as_ref(){
            "host"=>{
                host = args.nth(0).unwrap().clone();
                it_count+=1;
                },
            "static"=>{ 
                static_content = args.nth(0).unwrap().clone();
                it_count+=1;
            },
            "property"=>{},
            _=>{},
        }
    }
    ServerInstance::new(host, static_content).run_server();
}