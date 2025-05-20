fn main() {
    let mut input = String::new();

    while input.trim() != "#" {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Ошибка при чтении ввода.");
        println!("Содержимое input: {:#?}", input);
    }
}
