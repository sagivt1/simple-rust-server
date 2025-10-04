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

    if let Err(e) = stream.read(&mut buffer){
        eprintln!("Failed to read stream: {}", e);
        return;
    }

    // Get the requested path
    // typically start with "GET /path HTTP/1.1..."
    let get = b"GET / HTTP/1.1\r\n";

    // check if the start of the buffer match the request for the root path
    let (status_line, filename) = if buffer.starts_with(get) {
        // Success -> root path was requested
        ("HTTP/1.1 200 OK\r\n", "hello.html")
    } else {
        // Error -> any other path
        ("HTTP/1.1 404 NOT FOUND\r\n", "404.html")
    };

    // Reading html content from file
    let contents = std::fs::read_to_string(filename).expect("Could not read file");

    // Constructing the response  
    let respone = format!(
        "{}Content-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );   

    // Write the full response to the stream
    if let Err(e) = stream.write_all(respone.as_bytes()) {
        eprintln!("Failed to write to stream: {}", e)
    } 

    // flush the stream to ensure all the data is send immediately.
    if let Err(e) = stream.flush() {
         eprintln!("Failed to flush stream: {}", e);
    }

    println!("Request handled, response sent.");
   
}
