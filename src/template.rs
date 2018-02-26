extern crate regex;
use self::regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};
use std::path::Path;
#[derive(Debug)]
pub struct Template<'a>{
    name: &'a str,
    data: HashMap<&'a str, &'a str>,
}

impl<'a> Template<'a> {
    pub fn new(name: &'a str, data: HashMap<&'a str, &'a str>) -> Template<'a> {
        Template {name: name, data: data}
    }
    pub fn render(self) -> String{
        let s = &self.name.to_string();
        let path = Path::new(&s);
        let mut html = String::new();
        match File::open(path) {
                Ok(file) => {
                    html = Template::reg(&self, BufReader::new(file));
                },
                Err(_) => {
                    println!("can't find {}", self.name);
                },
        }
        html
    }

    fn reg<R: Read>(&self, br: BufReader<R>) -> String{
        let re = Regex::new(r"<%=[\s]*(.*?)[\s]*%>").unwrap();
        let mut result = String::new();
        println!("start");
        'outer: for xs in br.lines() {
            let s = xs.unwrap() + "\n";
            for cap in re.captures_iter(&s) {
                match self.data.get(&cap[1]) {
                    Some(ref d) => {
                        result += &s.to_string().replace(&cap[0], &d);
                        continue 'outer;
                    },
                    _ => {
                        println!("can't find {}", &cap[1])
                    },
                }
            }
            result += &s;
        }
        result
    }
}
