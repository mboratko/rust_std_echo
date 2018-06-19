use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() -> io::Result<()> {
    println!("SERVER          (ctrl+c to quit)");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // accept connections and process them serially
    for stream in listener.incoming() {
        // The ? operator unwraps an Ok, and returns early if it is an Error
        handle_client(stream?)?;
    }
    Ok(())
}

fn handle_client(stream: TcpStream) -> io::Result<()> {
    while let Ok(_) = stream.peer_addr() {
        // Using io::BufReader will allow us to use the .read_line method,
        // storing data from the client until a newline is seen.
        let mut reader = io::BufReader::new(&stream);
        let mut received_str = "".to_string();
        reader.read_line(&mut received_str)?;
        let received_str = received_str.trim_right();
        println!("Client >>> {}", received_str);

        // Send response to client
        let mut writer = io::BufWriter::new(&stream);
        writer.write_fmt(format_args!("Thanks for all the {}!\n", received_str))?;
        // io::BufWriter is flushed (and therefore data is sent)
        // when it goes out of scope
    }
    Ok(())
}