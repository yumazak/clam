extern crate clam;
use clam::template;
use std::collections::HashMap;

fn main() {
    let s = "hi <% name %>";
    let mut data = HashMap::new();
    data.insert("name", "yuma");
    let result = template::reg(&s, data);
    println!("{}", result);
}
