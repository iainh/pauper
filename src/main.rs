#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate clap;
extern crate rocket;

use clap::{Arg, ArgMatches};

use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket::State;

fn parse_args() -> ArgMatches<'static> {
    app_from_crate!()
        .arg(Arg::with_name("DOCROOT")
            .short("d")
            .takes_value(true)
            .required(true))
        .get_matches()
}

#[get("/")]
fn index(docroot: State<String>) -> io::Result<NamedFile> {
    NamedFile::open(format!("{}/{}", docroot.as_str(), "index.html"))
}

#[get("/<file..>")]
fn files(file: PathBuf, docroot: State<String>) -> Option<NamedFile> {
    NamedFile::open(Path::new(docroot.as_str()).join(file)).ok()
}

fn rocket(docroot: &str) -> rocket::Rocket {
    rocket::ignite()
        .manage(format!("{}", docroot))
        .mount("/", routes![index, files])
}

fn main() {
    let args = parse_args();

    let docroot = args.value_of("DOCROOT").unwrap();

    println!("Docroot: {}", docroot);

    rocket(docroot)
        .launch();
}
