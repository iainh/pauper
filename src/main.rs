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
use rocket::config::{Config, Environment};

fn parse_args() -> ArgMatches<'static> {
    app_from_crate!()
        .arg(Arg::with_name("DOCROOT")
            .short("d")
            .long("root")
            .takes_value(true))
        .arg(Arg::with_name("PORT")
            .short("p")
            .long("port")
            .takes_value(true))
        .arg(Arg::with_name("VERBOSE")
            .short("v")
            .long("verbose"))
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
    let full_path = PathBuf::from(doc_root.as_str()).join(&file);
    let file_path = if full_path.is_dir() {
        file.join("index.html")
    } else {
        file
    };
    get_named_file(&doc_root, &file_path).ok()
}

fn rocket(config: Config, logging: bool, doc_root: &str) -> rocket::Rocket {
    rocket::custom(config, logging)
        .manage(format!("{}", doc_root))
        .mount("/", routes![index, files])
}

fn main() {
    let args = parse_args();

    let current_dir = env::current_dir().unwrap();
    let cwd = current_dir.to_str().unwrap();

    let doc_root = args.value_of("DOCROOT").unwrap_or(cwd);

    let port = args.value_of("PORT").unwrap_or("8000");
    let logging = args.occurrences_of("VERBOSE") > 0;

    let config = Config::build(Environment::Development)
        .port(port.parse::<u16>().unwrap())
        .finalize();
    rocket(config.unwrap(), logging, doc_root)
        .launch();
}
