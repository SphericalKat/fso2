use std::{path::PathBuf, env};
use askama::Template;
use rocket::{Outcome, Rocket, request::FromRequest, response::NamedFile, http::Status};
use walkdir::{DirEntry, Error, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn strip_prefix(e: Result<DirEntry, Error>) -> Option<PathBuf> {
    let path = e.unwrap();
    return if path.path().exists() {
        Some(path.path().strip_prefix(env::current_dir().unwrap()).unwrap().to_path_buf())
    } else {
        None
    }
}

struct TemplateContainer {
    path: String,
    filename: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct DirTemplate {
    containers: Vec<TemplateContainer>,
}

pub struct CustomPath(pub PathBuf);

impl <'a, 'r> FromRequest<'a, 'r> for CustomPath {
    type Error = ();

    fn from_request(request: &'a rocket::Request<'r>) -> rocket::request::Outcome<Self, Self::Error> {
        let path_str = request.uri().path().strip_prefix("/");
        if path_str.is_none() {
            return Outcome::Failure((Status::InternalServerError, ()));
        }

        let path = PathBuf::from(path_str.unwrap());
        if path.exists() {
            return if path.is_file() {
                Outcome::Success(CustomPath(path))
            } else {
                Outcome::Forward(())
            }
        } else {
            Outcome::Failure((Status::NotFound, ()))
        }
    }
}

#[get("/<file..>")]
fn static_file(file: PathBuf, _path: CustomPath) -> Option<NamedFile> {
    NamedFile::open(file).ok()
}

#[get("/<file..>", rank = 2)]
fn file(file: PathBuf) -> DirTemplate {
    let walker = WalkDir::new(file).max_depth(1).into_iter();

    let files = walker
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|f| f.ok())
        .map(|f| f.path().to_path_buf());

    let containers = files
        .filter_map(|f| {
            if f.file_name().is_none() {
                None
            } else {
                Some(TemplateContainer {
                    filename: f.file_name().unwrap().to_string_lossy().to_string(),
                    path: f.to_string_lossy().to_string(),
                })
            }
        })
        .collect::<Vec<TemplateContainer>>();

    DirTemplate { containers }
}

pub fn fuel(rocket: Rocket) -> Rocket {
    rocket.mount("/", routes![static_file, file])
}