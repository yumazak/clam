extern crate regex;
use func::*;

pub struct StrReplace {
    data: String,
}

impl StrReplace {
    pub fn new(str: &str) -> StrReplace { StrReplace { data: str.to_string() } }
    pub fn replace(&mut self, search: &str, replacement: &str) -> &mut Self {
        self.data = self.data.replace(search, replacement);
        self
    }
    pub fn to_str(&self) -> &str {&self.data}
    pub fn delete(&mut self) -> &mut Self {
        self.data = "".to_string();
        self
    }
}

pub struct Stack<'a> {
    pub list: Vec<Command<'a>>,
    pub data: Vec<String>,
}

impl<'a> Stack<'a> {
    pub fn add(&mut self, line: &str) -> &mut Self{
        match self.data.pop() {
            Some(mut v) => {
                v.push_str(line);
                self.data.push(v.to_string())
            },
            None => {self.data.push(line.to_string());}
        }
        self
    }
}
