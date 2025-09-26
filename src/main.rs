use std::net::TcpListener;

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
                println!("Connection established!");
                // todo: Handel Connection 
            }
            Err(e) => {
                // log the error and continue listening for the next connection
                println!("Connection failed: {}", e);
            }
        }
    }

}
