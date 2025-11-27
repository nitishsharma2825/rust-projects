use std::{io::{BufRead, BufReader, Write}, net::TcpListener};
use redis_cmd::common::{Request, Response};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8082").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let reader = BufReader::new(stream.try_clone().unwrap());

        for line in reader.lines() {
            let line = line.unwrap();
            let request: Request = serde_json::from_str(&line).unwrap();

            match request {
                Request::Get { key } => {
                    println!("Get {key}");
                    let resp = Response::Ok(None);
                    writeln!(stream, "{}", serde_json::to_string(&resp).unwrap()).unwrap();
                }
                Request::Set { key, value } => {
                    println!("Set {key} {value}");
                    let resp = Response::Ok(Some(value));
                    writeln!(stream, "{}", serde_json::to_string(&resp).unwrap()).unwrap();
                }
                Request::Rm { key } => {
                    println!("Rm {key}");
                    let resp = Response::Ok(None);
                    writeln!(stream, "{}", serde_json::to_string(&resp).unwrap()).unwrap();
                }
            }
        }
    }
}
