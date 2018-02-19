extern crate regex;
use self::regex::Regex;
use std::collections::HashMap;
pub fn draw(html: &str) -> String {
    println!("{}", html);
    return html.to_string()
}
pub fn reg(str: &str, data: HashMap<&str, &str>) -> String {
    let re = Regex::new(r"<%[\s]*(.*?)[\s]*%>").unwrap();
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
