use actix_web::{get, web, HttpResponse, Responder};
use askama::Template;
use std::{env, path::PathBuf};
use walkdir::{DirEntry, Error, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn strip_prefix(e: Result<DirEntry, Error>) -> PathBuf {
    e.unwrap()
        .path()
        .strip_prefix(env::current_dir().unwrap())
        .unwrap()
        .to_path_buf()
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

#[get("/")]
pub async fn render_files_root() -> impl Responder {
    let current_dir = env::current_dir().unwrap();
    let walker = WalkDir::new(current_dir.clone()).max_depth(1).into_iter();
    let files = walker
        .filter_entry(|e| !is_hidden(e))
        .map(|e| strip_prefix(e));
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

    let template = DirTemplate { containers };
    HttpResponse::Ok().body(template.render().unwrap())
}

#[get("/{path:.*}")]
pub async fn render_files(path_param: web::Path<(String,)>) -> impl Responder {
    let path = path_param.into_inner().0;
    let current_dir = env::current_dir().unwrap();
    let walker = WalkDir::new(current_dir.join(path))
        .max_depth(1)
        .into_iter();
    let files = walker
        .filter_entry(|e| !is_hidden(e))
        .map(|e| strip_prefix(e));
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

    let template = DirTemplate { containers };
    HttpResponse::Ok().body(template.render().unwrap())
}
