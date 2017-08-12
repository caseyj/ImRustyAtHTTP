/*
*This file defines a transport type used to transmit a completed 
*   HTTP request and its source to the processing "server" which will  
*   format and conduct a response
*/


use std::net::{TcpStream};

pub struct StreamMessage{
    pub stream: TcpStream,
    pub message: String,
}

impl StreamMessage{
    pub fn new(streamer: TcpStream, msg: String)->StreamMessage{
        StreamMessage{
            stream: streamer,
            message: msg,
        }
    }
}