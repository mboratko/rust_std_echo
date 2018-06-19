use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> io::Result<()> {
    println!("CLIENT          (ctrl+c to quit)");
    // Connect to server on localhost port 7878
    let stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    loop {
        {
            // Get input from user
            print!("Server <<< ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            // Write input to buffer
            let mut writer = io::BufWriter::new(&stream);
            writer.write(&input.as_bytes())?;
            // io::BufWriter is flushed (and therefore data is sent)
            // when it goes out of scope
        }
        // Read input from server into buffer
        let mut reader = io::BufReader::new(&stream);
        let mut received_str = "".to_string();
        reader.read_line(&mut received_str)?;
        println!("Server >>> {}", received_str.trim_right());
    }
}