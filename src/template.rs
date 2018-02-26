extern crate regex;
use self::regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};
use std::path::Path;

struct Template {
    name: &str,
    data: HashMap<&str, &str>,
}

impl Template {
    fn render(data: HashMap<&str, &str>, fname: &str) -> String{
        let s = fname.to_string();
        let path = Path::new(&s);
        let mut html = String::new();
        match File::open(path) {
                Ok(file) => {
                    html = Template::reg(BufReader::new(file), data);
                },
                Err(_) => {
                    println!("can't find {}", fname);
                },
        }
        return html;
    }

    fn reg<R: Read>(br: BufReader<R>, data: HashMap<&str, &str>) -> String{
        let re = Regex::new(r"<%=[\s]*(.*?)[\s]*%>").unwrap();
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
}
