# ImRustyAtHTTP
A basic HTTP Server in Rust to play a bit with the language and server concepts


## Intention:
I found an interesting post on Reddit: https://www.reddit.com/r/dailyprogrammer/comments/6lti17/20170707_challenge_322_hard_static_http_server/ and I immediately began hacking away at the problem. The more I implemented, the more I found to explore and decided to take this opportunity 
to see not only how far I could go but what I could comparably implement against some of the most popular HTTP Server frameworks out there.
I dont see this as a competitor to the likes of Flask or Springboot, but absolutely see this as a personal and professional challenge on what my limits of imagination allow. 

## What I have already done:
### So far I have implemented the following features:
* Read basic HTTP requests
* Return given files requested by the browser
* Map basic HTTP header fields to an object for easy processing
* Return a GET request
* Return a POST request
* Capture parameters passed by a GET request
* Capture parameters passed by a POST request
* Utilizing Rust's channels and safe multithreading to handle request and response behavior.
* Building a channel to pass parsed HTTP requests to processing rather than copying the server

## Goals for what comes next:
### Some of these interest me as bonus challenges
* Implement features of HTTP I have yet to consider, such as adhering to some browser requests
* Building a means for a framework interaction with the server
* The above involves allowing for a "registration function" which would allow a user to map their own function to a request parameter
* Implement more features of HTTP which require better logic than what I use (actually doing more than 404/200 responses)
* Create a means by which a person using this server could create security logic, some sort of universal is allowed function.

