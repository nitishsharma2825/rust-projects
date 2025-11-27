use std::{io::{BufReader, BufWriter, Read, Write}, net::TcpStream};

fn main() {
    let server = TcpStream::connect("127.0.0.1:8082").unwrap();
    let mut writer = BufWriter::new(server.try_clone().unwrap());
    let mut reader = BufReader::new(server);

    let request = String::from("PING");
    let bytes = request.as_bytes();
    let len = bytes.len() as u8;

    writer.write_all(&len.to_le_bytes()).unwrap();
    writer.write_all(bytes).unwrap();
    writer.flush().unwrap();

    let mut response_len = [0u8; 1];
    reader.read_exact(&mut response_len).unwrap();

    let mut response_bytes = vec![0u8; u8::from_le_bytes(response_len) as usize];
    reader.read_exact(&mut response_bytes).unwrap();

    println!("{:?}", String::from_utf8(response_bytes).unwrap());
}
