#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Request {
    Set { key: String, value: String},
    Get { key: String },
    Rm {key: String},
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Response {
    Ok(Option<String>),
    Err(String),
}