use std::fmt::Display;

pub struct Statistics {
    header: String,
    values: String
}

impl Statistics {
    pub fn new() -> Self {
        Self { header: "|".to_owned(), values: "|".to_owned() }
    }

    pub fn add_str(&mut self, key: &str, value: &str) {
        let key_len = key.chars().count();
        let value_len = value.chars().count();
        let max_len = key_len.max(value_len);

        self.header.push_str(&format!(" {}{} |", key, " ".repeat(max_len - key_len)));
        self.values.push_str(&format!(" {}{} |", value, " ".repeat(max_len - value_len)));
    }

    pub fn add(&mut self, key: &str, value: &dyn Display) {
        self.add_str(key, &format!("{}", value));
    }

    pub fn print(&self) {
        println!("{}\n{}", self.header, self.values);
    }
}
