use std::{io::{BufRead, BufReader, Write}, net::TcpStream};
use redis_cmd::common::{Request, Response};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8082").unwrap();

    // send commands, each followed by newline
    for cmd in [
        Request::Get { key: "nitish".into() },
        Request::Set { key: "nitish".into(), value: "sharma".into() },
        Request::Rm  { key: "nitish".into() },
    ] {
        let json = serde_json::to_string(&cmd).unwrap();
        writeln!(stream, "{}", json).unwrap();
    }

    stream.flush().unwrap();

    // read responses
    let reader = BufReader::new(stream);

    for line in reader.lines() {
        let line = line.unwrap();
        let response: Response = serde_json::from_str(&line).unwrap();
        println!("response: {:?}", response);
    }
}
