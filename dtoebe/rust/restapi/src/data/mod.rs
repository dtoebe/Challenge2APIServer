extern crate serde_derive;

use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub body: String,
}

impl Data {
    pub fn new(json: String) -> Result<Data, Box<Error>> {
        Ok(Data {
            id: 1234,
            name: "test".to_string(),
            email: "no@email.com".to_string(),
            body: "Foo bar".to_string(),
        })
    }
}
