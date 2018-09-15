extern crate serde_derive;
extern crate serde_json as json;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

#[derive(Serialize, Deserialize, Clone)]
pub struct Comment {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub body: String,
}

impl Comment {
    pub fn new(name: String, email: String, body: String) -> Comment {
        Comment {
            id: 123,
            name: name,
            email: email,
            body: body,
        }
    }

    fn new_with_id(id: u32, name: String, email: String, body: String) -> Comment {
        Comment {
            id: id,
            name: name,
            email: email,
            body: body,
        }
    }
}

pub struct Data {
    pub comments: HashMap<u32, Comment>,
    pub file_path: String,
}

impl Data {
    pub fn new(path: String) -> Result<Data, Box<Error>> {
        let file = File::open(&path)?;
        let com: Vec<Comment> = json::from_reader(file)?;
        let mut comments = HashMap::new();

        for c in com {
            comments.insert(c.id, c);
        }

        Ok(Data {
            comments: comments,
            file_path: path,
        })
    }

    pub fn add_comment(&mut self, comment: Comment) -> Comment {
        self.comments.insert(comment.id, comment.clone());
        comment
    }

    pub fn update_comments(
        &mut self,
        id: u32,
        name: String,
        email: String,
        body: String,
    ) -> Result<Comment, &'static str> {
        match self.comments.get(&id.clone()) {
            Some(_) => {
                self.comments.insert(
                    id.clone(),
                    Comment::new_with_id(id.clone(), name, email, body),
                );
                Ok(self.comments[&id].clone())
            }
            None => Err("id not found"),
        }
    }

    pub fn delete_comments(&mut self, comment: Comment) -> Result<Comment, Box<Error>> {
        self.comments.remove(&comment.id);
        Ok(comment)
    }
}
