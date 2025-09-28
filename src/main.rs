use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

// Main function for the web server
fn main() {
    // Define address to bind "127.0.0.1:7878" (localhost) with port 7878.
    let listener = match TcpListener::bind("127.0.0.1:7878"){
        Ok(listener) => {
            // if the bind succsued get TcpListener
            println!("Server listening on http://127.0.0.1:7878");
            listener
        }
        Err(e) => {
            // if bind fails print the error message and panic
            eprintln!("Failed to bind to address: {}", e);
            panic!("Could not start the server, Exiting");
        }
    };

    // Wait for connection -> when recive connection return Result<TcpStream, E>
    for stream in listener.incoming() {
        // If connection succeseful, get TcpStream
        // if connection fails, panic and crash
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                // log the error and continue listening for the next connection
                println!("Connection failed: {}", e);
            }
        }
    }

}

// process incoming client connection
fn handle_connection(mut stream : TcpStream) {

    // buffer to hold incoming data (https request)
    let mut buffer = [0; 1024];

    // read data from strem
    match stream.read(&mut buffer) {
        Ok(_) => {
            //convert buffer to readable string
            let request = String::from_utf8_lossy(&buffer[..]);
            println!("request:\n{}", request);
        }
        Err(e) => {
            eprintln!("Failed to read stream: {}", e);
        }
    }
}
