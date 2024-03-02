#![allow(dead_code)]

pub mod account;
use actix_web::{HttpResponse, Responder};
use std::fs;
use std::io::{BufReader, Read, Result};
use askama::Template;


#[derive(Template)]
#[template(path="index.html")]
struct IndexTemplate<'a>{
    name:&'a str
}


pub async fn home() -> impl Responder {
    // HttpResponse::Ok().body(load_html("index.html").unwrap())

    let html = IndexTemplate{name:"world"};
    HttpResponse::Ok().body(html.render().unwrap())
}

pub fn load_html(html_file: &str) -> Result<String> {
    let file = fs::File::open(format!("static/html/{}", html_file))?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}
