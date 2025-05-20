// use std::cmp;
fn main() {
    let mut input = std::io::stdin().lines();
    // let x = input.next().unwrap().unwrap();
    let start: i32 = input.next().unwrap().unwrap().trim().parse().expect("Not u8 number");
    let end: i32 = input.next().unwrap().unwrap().trim().parse().expect("Not u8 number");
    // let b:i16 = input.next().unwrap().unwrap().trim().parse().expect("Not u8 number");
    let mut str: String = String::from("");

    for iter in start..=end {
        if iter != start {
            str += " | ";
        }
        str += iter.to_string().as_str();
    }
    println!("{}", str);
}
