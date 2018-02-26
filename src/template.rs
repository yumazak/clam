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
use util::StrReplace;


//#[derive(Debug)]
pub struct Template<'a>{
    name: &'a str,
    data: HashMap<&'a str, &'a str>,
    hash_maps: HashMap<&'a str, HashMap<&'a str, &'a str>>,
}
pub struct TemplateBuilder<'a> {
    name: &'a str,
    data: HashMap<&'a str, &'a str>,
    hash_maps: HashMap<&'a str, HashMap<&'a str, &'a str>>,
}

impl<'a> TemplateBuilder<'a> {
    pub fn new(name: &'a str) -> TemplateBuilder<'a> {
        let s = "**default";
        let mut d = HashMap::new();
        d.insert(s, s);
        let mut d2 = HashMap::new();
        let mut d3 = HashMap::new();
        d2.insert(s, s);
        d3.insert(s, d2);
        TemplateBuilder{name: name, data: d, hash_maps: d3}
    }
    pub fn data(&mut self, data: HashMap<&'a str, &'a str>) -> &mut TemplateBuilder<'a> {
        self.data = data;
        self
    }
    pub fn hash_maps(&mut self, hash_maps: HashMap<&'a str, HashMap<&'a str, &'a str>>) -> &mut TemplateBuilder<'a> {
        self.hash_maps = hash_maps;
        self
    }
    pub fn build(&self) -> Template<'a> {
        Template{name: &self.name, data: self.data.clone(), hash_maps: self.hash_maps.clone()}
    }
}

impl<'a> Template<'a> {
    pub fn new(name: &'a str, data: HashMap<&'a str, &'a str>,
        hash_maps: HashMap<&'a str, HashMap<&'a str, &'a str>>) -> Template<'a> {
        Template {name: name, data: data, hash_maps: hash_maps}
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
        let re = Regex::new(r"<%=[\s]*(.*?)[\s]*%>").unwrap();
        let mut result = String::new();
        'outer: for xs in br.lines() {
            let s = xs.unwrap() + "\n";
            let mut line = StrReplace::new(&s);
            for cap in re.captures_iter(&s) {
                match self.data.get(&cap[1]) {
                    Some(d) => {
                        line.replace(&cap[0], d);
                    },
                    _ => {
                        match self.hash_maps.get(&cap[1]) {
                            Some(_d) => {
                                println!("hash_maps");
                                continue 'outer;
                            },
                            _ => {println!("can't find {}", &cap[1])},
                        }
                    },
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
