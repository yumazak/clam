pub enum Command {
    For{var: String, start: u32, end: u32},
    If
}
pub struct StrReplace {
    data: String,
}

impl StrReplace {
    pub fn new(str: &str) -> StrReplace {
        StrReplace { data: str.to_string() }
    }
    pub fn replace(&mut self, search: &str, replacement: &str) -> &mut Self {
        self.data = self.data.replace(search, replacement);
        self
    }
    pub fn to_str(&self) -> &str {
        &self.data
    }
    pub fn delete(&mut self) -> &mut Self {
        self.data = "".to_string();
        self
    }
}

pub struct Stack {
    pub list: Vec<Command>,
    pub args: Vec<Vec<String>>,
    pub data: Vec<String>,
}

impl Stack {
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

    pub fn pop_all(&mut self) -> &mut Self{
        // match self.list.last() {
        //     Some(..) => {self.list.pop();},
        //     _ => {println!("list is already none")}
        // };
        // match self.args.last() {
        //     Some(..) => {self.args.pop();},
        //     _ => {println!("list is already none")}
        // };
        // match self.args.last() {
        //     Some(..) => {self.args.pop();},
        //     _ => {println!("list is already none")}
        // };
        self.list.pop();
        self.args.pop();
        self.data.pop();
        self
    }
}
