use std::ops::Drop;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Appellation {
    name: String,
    nicknames: Vec<String>
}

impl Appellation {
    pub fn new(name: String, nicknames: Vec<String>) -> Self {
        Self {name, nicknames}
    }
}

impl Drop for Appellation {
    fn drop(&mut self) {
        print!("Dropping {} ", self.name);
        if !self.nicknames.is_empty() {
            print!("AKA {}", self.nicknames.join(", "));
        }
        println!("");
    }
}
