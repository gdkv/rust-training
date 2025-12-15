use std::fs;

fn extract_errors(text: &str) -> Vec<String> {
    let text_array = text.split("\n").collect();
    let mut errors: Vec<String> = vec![];

    for item in text_array {
        if item.starts_with("ERR") {
            errors.push(item.to_string());
        }
    }

    errors
}
fn main() {
    let mut errors: Vec<String> = vec![];
    match fs::read_to_string("logs.txt") {
        Ok(contents) => errors = extract_errors(contents.as_str()),
        Err(_) => println!("Couldn't read logs.txt"),
    }

    println!("{:#?}", errors);
}