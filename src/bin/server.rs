use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) -> io::Result<()> {
    while let Ok(_) = stream.peer_addr() {
        let mut reader = io::BufReader::new(&stream);
        let mut writer = io::BufWriter::new(&stream);
        let mut received_str = "".to_string();
        reader.read_line(&mut received_str)?;
        let received_str = received_str.trim_right();
        println!("Client >>> {}", received_str);
        writer.write_fmt(format_args!("Thanks for all the {}!\n", received_str))?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}