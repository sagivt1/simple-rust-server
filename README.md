# Simple Rest Web Server

Basic web server for educatonal propeses build from scrath with rust.

## Features

- **TCP Listener:** Binds to `127.0.0.1:7878` and listens for HTTP requests.
- **Multithreading:** Spawns a new thread for **every incoming connection** to handle concurrent requests.
- **Basic Routing:** Serves `hello.html` for requests to the root path (`/`) and `404.html` for all others.
- **Raw I/O:** Reads and writes HTTP bytes directly using `std::net::TcpStream`.

## Getting Started

### Prerequisite
- [rust]