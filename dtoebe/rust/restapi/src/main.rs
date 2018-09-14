extern crate actix;
extern crate actix_web;
extern crate clap;

// extern crate data;
extern crate restapi;

use std::process::exit;

use actix_web::{http::Method, server, App};
use clap::{App as clapApp, Arg};

use restapi::data;
use restapi::handlers;

const HOST: &str = "127.0.0.1";

fn main() {
    let flags = clapApp::new("restapi")
        .version("0.1.0")
        .author("Daniel Toebe <dtoebe@protonmail.com")
        .about("simple REST API for the Copperhead Development Challenge")
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .help("port for the service to listen on")
                .takes_value(true)
                .default_value("3030"),
        ).arg(
            Arg::with_name("data_file")
                .short("f")
                .long("file")
                .help("the JSON data file to be loaded")
                .takes_value(true)
                .required(true),
        ).get_matches();

    let port = flags.value_of("port").unwrap_or_else(|| {
        println!("failed to get port flag");
        exit(1);
    });

    let sys = actix::System::new("Copperhead Challenge");
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(handlers::greet))
            .resource("/{name}", |r| r.f(handlers::greet))
            .resource("/comments", |r| {
                r.method(Method::GET).f(handlers::get_all_comments)
            }).resource("/comments", |r| {
                r.method(Method::POST).f(handlers::post_comment)
            }).resource("/comments/{id}", |r| {
                r.method(Method::GET).f(handlers::get_comment_by_id)
            }).resource("/comments/{id}", |r| {
                r.method(Method::PUT).f(handlers::put_comment)
            }).resource("/comments/{id}", |r| {
                r.method(Method::DELETE).f(handlers::delete_comment)
            })
    }).bind(&format!("{}:{}", HOST, port))
    .expect(&format!("cannot bind to {}:{}", HOST, port))
    .start();

    println!("Listening on {}:{}", HOST, port);
    let _ = sys.run();
}
