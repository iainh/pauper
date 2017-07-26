#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate clap;
extern crate rocket;

use clap::{Arg, ArgMatches};

use std::io;
use std::env;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket::State;

fn parse_args() -> ArgMatches<'static> {
    app_from_crate!()
        .arg(Arg::with_name("DOCROOT")
            .short("d")
            .takes_value(true))
        .get_matches()
}

fn get_named_file(doc_root: &String, file: &PathBuf) -> io::Result<NamedFile> {
    NamedFile::open(Path::new(doc_root).join(file))
}

#[get("/")]
fn index(doc_root: State<String>) -> io::Result<NamedFile> {
    get_named_file(&doc_root, &PathBuf::from("index.html"))
}

#[get("/<file..>")]
fn files(file: PathBuf, doc_root: State<String>) -> Option<NamedFile> {
    get_named_file(&doc_root, &file).ok()
}

fn rocket(doc_root: &str) -> rocket::Rocket {
    rocket::ignite()
        .manage(format!("{}", doc_root))
        .mount("/", routes![index, files])
}

fn main() {
    let args = parse_args();

    let current_dir = env::current_dir().unwrap();
    let cwd = current_dir.to_str().unwrap();

    let doc_root = args.value_of("DOCROOT").unwrap_or(cwd);
    rocket(doc_root)
        .launch();
}
