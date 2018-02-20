extern crate regex;
extern crate iron;
use self::regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};
use std::path::Path;
use self::iron::{headers, status};
use self::iron::modifiers::Header;
use self::iron::prelude::*;

//return IronResult<Response>
pub fn add(data: HashMap<&str, &str>, fname: &str) -> IronResult<Response>{
    let string = render(data,fname);
    Ok(Response::with((status::Ok, Header(headers::ContentType::html()), string)))
}

pub fn render(data: HashMap<&str, &str>, fname: &str) -> String{
    let s = fname.to_string() + ".clm";
    let path = Path::new(&s);
    let mut html = String::new();
    match File::open(path) {
            Ok(file) => {
                html = reg(BufReader::new(file), data);
            },
            Err(_) => {
                println!("can't find {}.clm", fname);
            },
    }
    return html;
}
fn reg<R: Read>(br: BufReader<R>, data: HashMap<&str, &str>) -> String{
    let re = Regex::new(r"<%[\s]*(.*?)[\s]*%>").unwrap();
    //let path: Path;
    let mut result = String::new();
    'outer: for xs in br.lines() {
        let s = xs.unwrap() + "\n";
        for cap in re.captures_iter(&s) {
            match data.get(&cap[1]) {
                Some(ref d) => {
                    let s = s.to_string().replace(&cap[0], &d);
                    result += &s;
                    continue 'outer;
                },
                _ => {
                    println!("can't find {}", &cap[1])
                },
            }
        }
        result += &s;
    }
    return result;
}
