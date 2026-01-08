use std::io::Error;

fn print_elements(elements: &[String]) -> Result<(), Error> {
    if elements.is_empty() {
        return Err(Error::other("Empty elements"));
    }

    for element in elements {
        println!("{}", element);
    }

    Ok(())
}

fn reverse_strings(items: &Vec<String>) -> Result<Vec<String>, Error> {
    if items.is_empty() {
        return Err(Error::other("Empty items"));
    }

    let mut reversed: Vec<String> = vec![];
    for item in items {
        reversed.push(make_string_reverse(item));
    }

    println!("Reversed: {:?}", reversed);

    Ok(reversed)
}

fn make_string_reverse(item: &String) -> String {
    let mut name_of_file: String = String::new();
    for char in item.chars().rev() {
        name_of_file += &char.to_string();
    }

    name_of_file
}

fn find_file_or(items: &Vec<String>, search: &str, fallback: &str) -> Result<String, Error> {
    items
        .iter()
        .find(|item| item.contains(search))
        .map_or(
            Ok(String::from(fallback)),
            |item| Ok(item.to_string())
        )
}

fn main() -> Result<(), Error> {
    let arr = vec![
        String::from("errors.txt"),
        String::from("access.txt"),
        String::from("debug.txt"),
    ];

    print_elements(&arr)?;
    reverse_strings(&arr)?;

    let find_file = find_file_or(&arr, "txt", "access.txt")?;
    println!("Found: {}", find_file);

    Ok(())
}