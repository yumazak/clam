extern crate regex;
use self::regex::Regex;
use std;
use std::collections::HashMap;
use util::StrReplace;
pub enum Command <'a>{
    For{var: String, start: u32, end: u32},
    If{con: String, val1: String, val2: String, map: HashMap<&'a str, &'a str>}
}
pub struct For {
    var: String,
    start: u32,
    end: u32
}
impl For {
    pub fn new(v: String, s: u32, e: u32) -> For {
        For{var: v, start: s, end: e}
    }

    pub fn run(&self, data: String) -> String {
        let r = std::ops::Range { start: self.start, end: self.end };
        let mut result = String::new();
        for i in r.enumerate() {
            result += &data;
        }
        result
    }
}

pub struct If <'a>{
    con: String,
    val1: String,
     val2: String,
     map: HashMap<&'a str, &'a str>,
}
impl<'a> If<'a> {
    pub fn new(c: String, v1: String, v2: String, m: HashMap<&'a str, &'a str>) -> If <'a> {
        If{con: c, val1: v1, val2: v2, map: m}
    }
    pub fn run(&self, data: String) -> String {
        let bl: bool;
        match self.con.as_str() {
            "==" => {
                let a = &self.map.get_mut(&self.val1);
                let b = &self.map.get(&self.val2);
            },
            "!=" => b = self.v1 != self.v2,
            _ => {panic!("can't find {}", self.c)}
        }
        println!("{}", b);
        if(b){
            return data
        }else{
            return "".to_string()
        }
    }
}
