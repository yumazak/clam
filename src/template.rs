extern crate regex;
extern crate iron;
use self::iron::status;
use self::iron::prelude::*;
use self::regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};
use std::path::Path;
use util::*;

pub struct Template<'a>{
    name: &'a str,
    data: HashMap<&'a str, &'a str>,
}
pub struct TemplateBuilder<'a> {
    name: &'a str,
    data: HashMap<&'a str, &'a str>,
}

impl<'a> TemplateBuilder<'a> {
    pub fn new(name: &'a str) -> TemplateBuilder<'a> {
        let s = "**default";
        let mut d = HashMap::new();
        d.insert(s, s);
        TemplateBuilder{name: name, data: d}
    }
    pub fn data(&mut self, data: HashMap<&'a str, &'a str>) -> &mut TemplateBuilder<'a> {
        self.data = data;
        self
    }
    pub fn build(&self) -> Template<'a> {
        Template{name: &self.name, data: self.data.clone()}
    }
}

impl<'a> Template<'a> {
    pub fn new(name: &'a str, data: HashMap<&'a str, &'a str>) -> Template<'a> {
        Template {name: name, data: data}
    }
    pub fn render(&self) -> String{
        let s = &self.name;
        let path = Path::new(&s);
        let mut html = String::new();
        match File::open(path) {
                Ok(file) => {
                    html = Template::reg(&self, BufReader::new(file));
                },
                Err(_) => {println!("can't find {}", self.name)},
        }
        html
    }

    fn reg<R: Read>(&self, br: BufReader<R>) -> String{
        let re = Regex::new(r"<%=\s*(.+?)\s*%>").unwrap();
        let mut result = String::new();
        'outer: for xs in br.lines() {
            let s = xs.unwrap() + "\n";
            let mut line = StrReplace::new(&s);
            for cap in re.captures_iter(&s) {
                match self.data.get(&cap[1]) {
                    Some(d) => {line.replace(&cap[0], d);},
                    _ => {},
                }
            }
            result += line.to_str();
        }
        result
    }

    pub fn html(&self) -> IronResult<Response>{
        let body = self.render();
        Ok(Response::with((status::Ok, mime!(Text/Html; Charset=Utf8), body)))
    }
}
