extern crate iron;
use self::iron::status;
use self::iron::prelude::*;
use template::Template;
use std::collections::HashMap;

pub fn html(fname: &str, data: HashMap<&str, &str>) -> IronResult<Response>{
    let body = Template::new(fname, data).render();
    Ok(Response::with((status::Ok, mime!(Text/Html; Charset=Utf8), body)))
}
