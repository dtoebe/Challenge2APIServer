extern crate actix_web;
extern crate serde_json;

use actix_web::{HttpRequest, Responder};

// #[derive(Deserialize)]
// pub struct QueryString {
//     size: u32,
//     from: u32,
// }

pub fn greet(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", to)
}

pub fn get_all_comments(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", to)
}

pub fn get_comment_by_id(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", to)
}

pub fn post_comment(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", to)
}

pub fn put_comment(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", to)
}

pub fn delete_comment(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", to)
}
