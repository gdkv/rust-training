fn main() {
    let mut input = std::io::stdin().lines();
    let input_max: i32 = input.next().unwrap().unwrap().trim().parse().expect("Введите целое число.");
    let input_count: u8 = input.next().unwrap().unwrap().trim().parse().expect("Введите целое число.");
    let end: i32 = input.next().unwrap().unwrap().trim().parse().expect("Not u8 number");
    let mut count_max = 0;

    let mut inputs = String::new();
    for _ in 0..3 {
        std::io::stdin()
            .read_line(&mut inputs)
            .expect("Ошибка при чтении ввода.");

        let num = inputs.trim().parse().expect("Введите целое число.");
        if input_max < num { count_max += 1; }
        inputs.clear();
    }

    println!("{:?}", count_max);
}
