extern crate serde_derive;
extern crate serde_json as json;

use std::error::Error;
use std::fs::File;

#[derive(Serialize, Deserialize)]
pub struct Comment {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub body: String,
}

pub struct Data {
    pub comments: Vec<Comment>,
    pub file_path: String,
}

impl Data {
    pub fn new(path: String) -> Result<Data, Box<Error>> {
        let file = File::open(&path)?;
        let comments = json::from_reader(file)?;

        Ok(Data {
            comments: comments,
            file_path: path,
        })
    }
}

