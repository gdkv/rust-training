fn main() {
    // let mut user_char:[char; 5] = ['_'; 5];
    let mut input = std::io::stdin().lines();

    for _ in 0..5 {
        print!("{}", input.next().unwrap().unwrap().trim());
    }
}
