fn main() {
    let mut input: String = String::new();
    let mut num_count: u8 = 0;

    std::io::stdin()
        .read_line(&mut input)
        .expect("Ошибка при чтении ввода.");

    for ch in input.trim().chars() {
        if ch.is_numeric() {
            num_count += 1;
        }
    }

    // while input.trim() != stop.trim() {
    //     input.clear();
    //     std::io::stdin()
    //         .read_line(&mut input)
    //         .expect("Ошибка при чтении ввода.");
    //
    //     if input.trim() == stop.trim() {
    //         continue;
    //     }
    //
    //     let number: f64 = match input.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    //
    //     total_count += number;
    // }

    println!("{:.1}", total_count);
}