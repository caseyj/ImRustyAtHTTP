use std;
use std::io::prelude::*;
use std::net::{TcpListener};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::io::*;

use req_res::ReqResEngine;
use stream_message::StreamMessage;
use http_req::{HTTPRequest};
use router::{Router,HttpResponse};
use file_responder::{specific_file_finder};


pub fn slash(req: HTTPRequest)->HttpResponse{
	println!("Default mapping of the {} route to index.html", req.get_file().unwrap());
	return specific_file_finder("/index.html".to_owned());
}


pub fn client_handle(stream: Result<std::net::TcpStream>, send_stringx: Sender<StreamMessage>){
	match stream {
		Ok(mut stream) => {
		    let mut buffer = [0; 1024];
		    stream.read(&mut buffer).unwrap();
		    let request = std::str::from_utf8(&buffer).unwrap();

			//we got the request so send it to the server thread
			let msg = StreamMessage::new(stream, String::from(request));
			send_stringx.send(msg).unwrap();
		}
		Err(e) => {
			println!("Failure {}", e); 
		}
	}
}

#[allow(dead_code)]
pub struct ServerInstance{
    
    pub tcp_listener: TcpListener,
    pub registered_gets: Router,
    pub registered_posts: Router,
}

impl ServerInstance{
    pub fn new(run_loc: String)->ServerInstance{
        let listener = TcpListener::bind(run_loc).unwrap();
        let mut gets = Router::new();
        let mut posts = Router::new();
        gets.register_route("/".to_owned() , slash);
        posts.register_route("/".to_owned() , slash);
        
        ServerInstance{
            tcp_listener: listener,
            registered_gets: gets,
            registered_posts: posts,
        }
    }

    pub fn register_post_route(&mut self, route: String, route_function: fn(HTTPRequest)->HttpResponse){
        self.registered_posts.register_route(route, route_function);
    }
    pub fn register_get_route(&mut self, route: String, route_function: fn(HTTPRequest)->HttpResponse){
        self.registered_gets.register_route(route, route_function);
    }

    pub fn run_server(&self){
        let (send_stringx, recieve_stringx): (Sender<StreamMessage>, Receiver<StreamMessage>) = mpsc::channel();
        let mut engine =  ReqResEngine::new();
        engine.set_get_routes(self.registered_gets.clone());
        engine.set_post_routes(self.registered_posts.clone());

        thread::spawn(move || {engine.run(recieve_stringx)});
        // accept connections and process them paralell
        for stream in self.tcp_listener.incoming() {

            let send_str = send_stringx.clone();

            thread::spawn(move || {client_handle(stream, send_str)});	
        }
    }

}