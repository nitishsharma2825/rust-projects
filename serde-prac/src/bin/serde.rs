use std::{env::current_dir, fs::OpenOptions, io::{Seek, Write}, path::PathBuf};

use serde_json::Deserializer;


#[derive(serde::Serialize, serde::Deserialize, Debug)]
enum Command {
    Set { key: String, value: String},
    Get { key: String },
    Rm {key: String},
}

fn main() {
    let cmd1 = Command::Get { key: "nitish".to_string() };
    let cmd2 = Command::Set { key: "nitish".to_string(), value: "sharma".to_string() };
    let cmd3 = Command::Rm { key: "nitish".to_string() };

    let log_path = PathBuf::from(current_dir().unwrap()).join(PathBuf::from("data.json"));
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&log_path).unwrap();

    serde_json::to_writer(&file, &cmd1).unwrap();
    serde_json::to_writer(&file, &cmd2).unwrap();
    serde_json::to_writer(&file, &cmd3).unwrap();

    file.flush().unwrap();

    file.seek(std::io::SeekFrom::Start(0)).unwrap();

    let mut stream = Deserializer::from_reader(&file).into_iter::<Command>();
    while let Some(cmd) = stream.next() {
        println!("{:?}", cmd.unwrap());
    }
}
