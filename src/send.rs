extern crate iron;
// extern crate hyper;
// use self::hyper::mime::{TopLevel, SubLevel, Attr, Value};

use self::iron::status;
use self::iron::prelude::*;
use template;
use std::collections::HashMap;

pub fn html(fname: &str, data: HashMap<&str, &str>) -> IronResult<Response>{
    let body = template::render(data,fname);
    //Ok(Response::with((status::Ok, Header(headers::ContentType::css()), string)))
    Ok(Response::with((status::Ok, mime!(Text/Html; Charset=Utf8), body)))
}
