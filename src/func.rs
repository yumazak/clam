use std;
pub enum Command{
    For{var: String, start: u32, end: u32},
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
