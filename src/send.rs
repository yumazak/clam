extern crate iron;
// extern crate hyper;
// use self::hyper::mime::{TopLevel, SubLevel, Attr, Value};

use self::iron::status;
use self::iron::prelude::*;
use template;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;
pub enum ContentType {
    HTML,
    JS,
    CSS
}
pub fn html(data: HashMap<&str, &str>, fname: &str) -> IronResult<Response>{
    let body = template::render(data,fname);
    //Ok(Response::with((status::Ok, Header(headers::ContentType::css()), string)))
    Ok(Response::with((status::Ok, mime!(Text/Html; Charset=Utf8), body)))
}

pub fn mime(name: &str, content_type: ContentType) -> IronResult<Response>{
    let body = file_read(name);
    match content_type {
        ContentType::HTML => Ok(Response::with((status::Ok, mime!(Text/Html; Charset=Utf8), body))),
        ContentType::CSS => Ok(Response::with((status::Ok, mime!(Text/Javascript; Charset=Utf8), body))),
        ContentType::JS => Ok(Response::with((status::Ok, mime!(Text/Css; Charset=Utf8), body)))
    }
}

fn file_read(name: &str) -> String{
    let s = name.to_string();
    let path = Path::new(&s);
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", name, Error::description(&why)),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", name, Error::description(&why)),
        Ok(_) => s,
    }
}
