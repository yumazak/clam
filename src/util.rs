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
