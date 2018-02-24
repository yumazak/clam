extern crate clam;
extern crate iron;
use clam::{template, send};
use std::collections::HashMap;
use iron::prelude::*;

fn main() {
    fn top_handler(_: &mut Request) -> IronResult<Response> {
        let mut data = HashMap::new();
        data.insert("hi", "good morning");
        data.insert("url", "anihaya.tk");
        send::html("view/test.html", data)
    }
    let _server = Iron::new(top_handler).http("localhost:3000").unwrap();
    println!("Lisning on port 3000");
}

// fn main() {
//     fn top_handler(req: &mut Request) -> IronResult<Response> {
//         let mut data = HashMap::new();
//         data.insert("yuu", "bnb");
//         let mut resp = Response::with((status::Ok, template::render(data,"view/index")));
//         //let mut resp = Response::with((status::Ok, "<b>hi</>"));
//         resp.headers.set(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![])));
//         return Ok(resp);
//     }
//
//     fn greet_handler(req: &mut Request) -> IronResult<Response> {
//         use params::{Params, Value};
//         let map = req.get_ref::<Params>().unwrap();
//         return match map.find(&["name"]) {
//             Some(&Value::String(ref name)) => {
//                 Ok(Response::with(
//                     (status::Ok,
//                      format!("Hello {}", name).as_str())))
//             },
//             _ => Ok(Response::with((status::Ok, "Hello world")))
//         }
//     }
//
//     //Create Router
//     let mut router = Router::new();
//     router.get("/", top_handler, "index");
//     router.post("/greet", greet_handler, "greeting");
//     println!("Listen on localhost:3000");
//     Iron::new(router).http("localhost:3000").unwrap();
// }
