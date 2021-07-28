mod shape;
mod traffic_light;
mod utils;

use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str;
use std::thread;

/**
 * Handle the input stream, keep running until error occurred
 */
fn handle_client(mut stream: TcpStream) {
    // mutable data buffer with 50 bytes
    let mut data = [0 as u8; 50];
    // loop for everything read from input stream
    while match stream.read(&mut data) {
        // successfully matched with the return value of function stream.read()
        Ok(size) => {
            if size > 0 {
                // convert u8 array to &str, remove the last character('\x0a')
                let str = match str::from_utf8(&data[0..size - 1]) {
                    Ok(v) => v,
                    Err(_) => "Invalid UTF-8 sequence received.",
                };
                // print received content
                println!(
                    "Message received from {}: {}",
                    stream.peer_addr().unwrap(),
                    str
                );
                // echo everything received
                stream.write(&data[0..size]).unwrap();
                // return true on success
                true
            } else {
                false
            }
        }
        // handle error while reading data from input stream
        Err(_) => {
            // print exit message
            println!(
                "Error occurred while reading from input stream, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            // shutdown input stream
            stream.shutdown(Shutdown::Both).unwrap();
            // return false on error
            false
        }
    } {}
    println!("Client from {} disconnected.", stream.peer_addr().unwrap());
}

fn main() {
    // bind tcp listener to local port
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // print starting message
    println!("Server listening on port 3333");
    // loop for every incoming connection
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // print client information
                println!("New connection: {}", stream.peer_addr().unwrap());
                // spawn a new thread for each client
                thread::spawn(move || {
                    // actually handle the incoming stream
                    handle_client(stream)
                });
            }
            Err(e) => {
                // print error detail
                println!("Error: {}", e);
            }
        }
    }
    // close the socket server
    drop(listener);
}
