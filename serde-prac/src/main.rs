use std::{fs::{File, OpenOptions}, io::{BufRead, BufReader, Read, Seek, Write}};

use bson::{Document, serialize_to_document};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Move {
    direction: String,
    steps: i32,
}

fn _json_serde() {
    let a = Move { direction: String::from("up"), steps: 10};
    println!("{:?}", a);

    let mut f = File::create_new("./data.json").unwrap();
    
    // serialize it as json string and write to file
    let serialized = serde_json::to_string(&a).unwrap();
    // let mut writer = BufWriter::new(f);
    f.write_all(serialized.as_bytes()).unwrap();

    f.seek(std::io::SeekFrom::Start(0)).unwrap();

    // deserialize from json string in file to our struct
    let mut json = String::new();
    f.read_to_string(&mut json).unwrap();
    let b: Move = serde_json::from_str(&json).unwrap();

    println!("{:?}", b);
}

// json serde with prefix length
fn json_vec_serde_newline_delim() {
    let mut a_vec = Vec::<Move>::new();
    for i in 1..11 {
        a_vec.push(Move {direction: String::from("up"), steps: i});
    }

    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .open("./data.txt")
        .unwrap();

    for elem in a_vec {
        let json = serde_json::to_string(&elem).unwrap();
        let bytes = json.as_bytes();

        // json + newline
        f.write_all(bytes).unwrap();
        f.write_all(b"\n").unwrap();
    }

    drop(f);

    let reader = BufReader::new(File::open("data.txt").unwrap());
    for line in reader.lines(){
        let json = line.unwrap();
        let value: Move = serde_json::from_str(&json).unwrap();

        println!("{:?}", value);
    }
}

// json serde with prefix length
fn _json_vec_serde_length_prefix() {
    let mut a_vec = Vec::<Move>::new();
    for i in 1..11 {
        a_vec.push(Move {direction: String::from("up"), steps: i});
    }

    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .open("./data.txt")
        .unwrap();

    for elem in a_vec {
        let json = serde_json::to_string(&elem).unwrap();
        let bytes = json.as_bytes();
        let len = bytes.len() as u32;

        // length prefix + binary data
        f.write_all(&len.to_le_bytes()).unwrap();
        f.write_all(bytes).unwrap();
    }

    drop(f);

    let mut rfile = File::open("./data.txt").unwrap();
    loop {
        let mut len_buf = [0u8; 4];

        match rfile.read_exact(&mut len_buf) {
            Ok(()) => (),
            Err(_) => break, // EOF
        }

        let len = u32::from_le_bytes(len_buf);

        let mut buf = vec![0u8; len as usize];
        rfile.read_exact(&mut buf).unwrap();

        let json_str = String::from_utf8(buf).unwrap();
        let value: Move = serde_json::from_str(&json_str).unwrap();

        println!("{:?}", value);
    }
}

// bson has automatic framing since it includes doc size at beginning so can be directly used
// for sequence encoding/decoding of data structures
fn _bson_vec_serde() {
    let mut a_vec = Vec::<Move>::new();
    for i in 1..11 {
        a_vec.push(Move {direction: String::from("up"), steps: i});
    }

    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .open("./data.bson")
        .unwrap();

    for elem in a_vec {
        let document = serialize_to_document(&elem).unwrap();
        document.to_writer(&mut f).unwrap();
    }

    drop(f);

    let mut rfile = File::open("./data.bson").unwrap();
    loop {
        match Document::from_reader(&mut rfile) {
            Ok(doc) => {
                let m: Move = bson::deserialize_from_document(doc).unwrap();
                println!("Read: {:?}", m);
            }
            Err(_) => {
                break; // EOF
            }
        }
    }
}

fn _bin_serde() {
    let a = Move { direction: String::from("up"), steps: 10};
    println!("{:?}", a);

    // text bytes
    let json_serialized = serde_json::to_vec(&a).unwrap();
    println!("json vec serialize: {:?}", json_serialized);

    let json_deserialized = str::from_utf8(&json_serialized).unwrap();
    println!("json vec deserialze: {:?}", json_deserialized);

    // binary bytes
    let bin_serialized = bincode::serialize(&a).unwrap();
    println!("binary vec serialize: {:?}", bin_serialized);

    let bin_deserialized: Move = bincode::deserialize(&bin_serialized).unwrap();
    println!("json vec deserialze: {:?}", bin_deserialized);
}

fn _bin_vec_serde() {
    let mut a_vec = Vec::<Move>::new();
    for i in 1..11 {
        a_vec.push(Move {direction: String::from("up"), steps: i});
    }

    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .open("./data.txt")
        .unwrap();

    for elem in a_vec {
        let bytes = bincode::serialize(&elem).unwrap();
        let len = bytes.len() as u32;

        // length prefix + binary data
        f.write_all(&len.to_le_bytes()).unwrap();
        f.write_all(&bytes).unwrap();
    }

    drop(f);

    let mut rfile = File::open("./data.txt").unwrap();
    loop {
        let mut len_buf = [0u8; 4];

        match rfile.read_exact(&mut len_buf) {
            Ok(()) => (),
            Err(_) => break, // EOF
        }

        let len = u32::from_le_bytes(len_buf);

        let mut buf = vec![0u8; len as usize];
        rfile.read_exact(&mut buf).unwrap();

        let value: Move = bincode::deserialize(&buf).unwrap();

        println!("{:?}", value);
    }
}

fn main() {
    // json_serde();
    // bin_serde();

    json_vec_serde_newline_delim();
}
