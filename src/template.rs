extern crate regex;
use self::regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::io::{BufReader};
use std::io::{BufWriter, Write};

use std::path::Path;
pub fn render(data: HashMap<&str, &str>, fname: &str, p: &str){
    let s = fname.to_string() + ".clm";
    let path = Path::new(&s);
    match File::open(path) {
            Ok(file) => {
                let re = Regex::new(r"/(.+?).clm$").unwrap();
                let cap = re.captures(&s).unwrap();
                reg(BufReader::new(file), data, &cap[1], p);
            },
            Err(_) => {
                println!("can't find {}.clm", fname)
            },
    }
}
fn reg<R: Read>(br: BufReader<R>, data: HashMap<&str, &str>, fname: &str, p: &str){
    let re = Regex::new(r"<%[\s]*(.*?)[\s]*%>").unwrap();
    let s =[p, "/" ,fname, ".html"].concat();
    let path = Path::new(&s);
    //let path: Path;
    let mut bw;
    match File::create(path) {
        Ok(file) => {
            bw = BufWriter::new(file);
        },
        Err(_) => {
            println!("can't find {}", p);
            fs::create_dir_all(Path::new(&p)).unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            });
            bw = BufWriter::new(File::create(path).unwrap());
        }
    }
    'outer: for xs in br.lines() {
        let s = xs.unwrap() + "\n";
        for cap in re.captures_iter(&s) {
            match data.get(&cap[1]) {
                Some(ref d) => {
                    let s = s.to_string().replace(&cap[0], &d);
                    bw.write(s.as_bytes()).unwrap();
                    continue 'outer;
                },
                _ => {
                    println!("can't find {}", &cap[1])
                },
            }
        }
        bw.write(&s.as_bytes()).unwrap();
    }
}
