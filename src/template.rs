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
use func::*;
//#[derive(Debug)]
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
    pub fn new(name: &'a str, data: HashMap<&'a str, &'a str>,
        hash_maps: HashMap<&'a str, HashMap<&'a str, &'a str>>) -> Template<'a> {
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
        let command = Regex::new(r"<%\s(.+?)\s.*%>").unwrap();
        let mut result = String::new();
        let mut stack = Stack{list: Vec::new(), data: Vec::new()};
        'outer: for xs in br.lines() {
            let s = xs.unwrap() + "\n";
            let mut line = StrReplace::new(&s);
            for cap in re.captures_iter(&s) {
                match self.data.get(&cap[1]) {
                    Some(d) => {line.replace(&cap[0], d);},
                    _ => {},
                }
            }
            for cap in command.captures_iter(&s) {
                match &cap[1] {
                    "for" => {
                        let re = Regex::new(r"<%[^=]\s*for\s(.+)\sin\s(\d+)..(\d+)[\s%]|[%]>").unwrap();
                        let cap = re.captures(&s).unwrap();
                        let cmd = Command::For{var: cap.get(1).map_or("".to_string(), |m| m.as_str().to_string()),
                            start: cap.get(2).map(|m| m.as_str().parse::<u32>().unwrap()).unwrap(),
                            end: cap.get(3).map(|m| m.as_str().parse::<u32>().unwrap()).unwrap()};
                        stack.list.push(cmd);
                        stack.data.push("".to_string());
                    },
                    "if" => {
                        let re = Regex::new(r"<%[^=]\s*if\s(.+)\s(.{2})\s([^\s%]+)[\s%]|[%]>").unwrap();
                        let cap = re.captures(&s).unwrap();
                        let cmd = Command::If{con: cap.get(2).map_or("".to_string(), |m| m.as_str().to_string()),
                            val1: cap.get(1).map_or("".to_string(), |m| m.as_str().to_string()),
                            val2: cap.get(3).map_or("".to_string(), |m| m.as_str().to_string()),
                            map: self.data};
                        stack.list.push(cmd);
                        stack.data.push("".to_string());
                    },
                    "end" => {
                        match stack.list.pop().unwrap() {
                            Command::For{var: v, start: s, end: e} => {
                                let f = For::new(v, s, e);
                                let html = f.run(stack.data.pop().unwrap());
                                result += &html;
                            },
                            Command::If{con: c, val1: v1, val2: v2, map: m} => {
                                let i = If::new(c, v1, v2, m);
                                let html = i.run(stack.data.pop().unwrap());
                                println!("html is {}", html);
                                result += &html;
                            },
                            _ => {println!("can't find command")}
                        };
                    },
                    _ => {println!("can't find command {}", &cap[1])}
                }
                line.delete();
            }
            if (stack.list.len() == 0) {
                result += line.to_str();
            } else {
                stack.add(line.to_str());
            }
            println!("{:?}", stack.data);

        }
        result
    }

    pub fn html(&self) -> IronResult<Response>{
        let body = self.render();
        Ok(Response::with((status::Ok, mime!(Text/Html; Charset=Utf8), body)))
    }
}
