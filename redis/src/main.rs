use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str;









/**
    #1 Bind to a port [X]
    In this stage, you'll start a TCP server on port 6379, which is
    the default port that Redis uses.

    #2 Respond to PING [ ]
    In this stage, you'll respond to the PING command. You'll
    use the Redis protocol to encode the reply.
*/


fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) { // while true do nothing
        Ok(size) => {
            // Echo everything
            println!("begin::'{}'::end", str::from_utf8(&data[0..size]).unwrap());
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 6379");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move | | {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                // connection failed
            }
        }
    }
    drop(listener);
}
