extern crate regex;
use self::regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn draw(html: &str) -> String {
    println!("{}", html);
    return html.to_string()
}
pub fn reg(str: &str, data: HashMap<&str, &str>, fname: &str) -> String {
    let re = Regex::new(r"<%[\s]*(.*?)[\s]*%>").unwrap();
    match File::open(&args[2]) {
            Ok(file) => find_string(BufReader::new(file), &args[1]),
            Err(_) => println!("{} not found", &args[1])
        }
    let mut result: String = "".to_string();
    for cap in re.captures_iter(str) {
        match data.get(&cap[1]) {
            Some(ref d) => {
                result = str.to_string().replace(&cap[0], &d);
            }
            _ => {
                println!("{}", &cap[1])
            }
        }
    }
    return result
}
