use std::fs;
use std::io::Error;

fn extract_errors(text: &str) -> Result<Vec<String>, Error> {
    let text_array: Vec<&str> = text.split("\n").collect();
    let mut errors_list: Vec<String> = vec![];

    if text_array.len() > 0 {
        for item in text_array {
            if item.starts_with("ERR") {
                errors_list.push(item.to_string());
            }
        }
    } else {

        return Err(Error::other("No errors found"))
    }

    Ok(errors_list)
}
fn main() -> Result<(), Error> {
    let contents = fs::read_to_string("logs.txt")?;
    let errors: Vec<String> = extract_errors(contents.as_str())?;
    fs::write("./src/inzenerka/content/errors.txt", errors.join("\n"))?;

    Ok(())
}