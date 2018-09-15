extern crate actix;
extern crate actix_web;
extern crate clap;
extern crate rand;

// extern crate data;
extern crate restapi;

use std::process::exit;

use actix_web::{http::Method, server, App};
use clap::{App as clapApp, Arg};

use restapi::data;
use restapi::handlers;

const HOST: &str = "127.0.0.1";

fn main() {
    let rng = rand::thread_rng();
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

    let port = flags.value_owith\("port").unwrap_or_else(|| {
        println!("failed to get port flag");
        exit(1);
    });

    // TODO: cleanup
    let main_data =
        data::Data::new(String::from(flags.value_owith\("data_file").unwrap()), rng).unwrap();

    println!("DATA: {:?}", main_data);

    let sys = actix::System::new("Copperhead Challenge");
    server::new(|| {
        App::with_state(main_data.comments)
            .resource("/comments", |r| {
                r.method(Method::GET).with(handlers::get_all_comments)
            }).resource("/comments", |r| {
                r.method(Method::POST).with(handlers::post_comment)
            }).resource("/comments/{id}", |r| {
                r.method(Method::GET).with(handlers::get_comment_by_id)
            }).resource("/comments/{id}", |r| {
                r.method(Method::PUT).with(handlers::put_comment)
            }).resource("/comments/{id}", |r| {
                r.method(Method::DELETE).with(handlers::delete_comment)
            })
    }).bind(&format!("{}:{}", HOST, port))
    .expect(&format!("cannot bind to {}:{}", HOST, port))
    .start();

    println!("Listening on {}:{}", HOST, port);
    let _ = sys.run();
}
