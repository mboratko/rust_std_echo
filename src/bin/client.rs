use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    loop {
        {
            print!("Server <<< ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let mut writer = io::BufWriter::new(&stream);
            writer.write(&input.as_bytes())?;
        }
        let mut reader = io::BufReader::new(&stream);
        let mut received_str = "".to_string();
        reader.read_line(&mut received_str)?;
        println!("Server >>> {}", received_str.trim_right());
    }
}