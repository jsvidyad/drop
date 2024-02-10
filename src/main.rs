mod appellation;

use appellation::Appellation;

fn main() {
    test_drop();
}

fn test_drop() {
    let name = "Jyothish".to_string();
    let mut nicknames = Vec::<String>::new();
    nicknames.push("Infi".to_string());
    nicknames.push("Thor".to_string());
    nicknames.push("Arak".to_string());
    let person = Appellation::new(name, nicknames);
    println!("{:?}", person);
}