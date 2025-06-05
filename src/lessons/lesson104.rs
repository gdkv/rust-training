fn main() {
    let mut input: String = String::new();
    // let mut num_count: u32 = 0;

    std::io::stdin()
        .read_line(&mut input)
        .expect("Ошибка при чтении ввода.");

    // let mut is_palindrome: bool = true;
    let mut ch1: Vec<&str> = input.trim().split("").collect();
    let ch2 = ch1.reverse();

    println!("{:?}", ch1);
    // for (x, y) in , input.trim().chars().rev() {
    //     println!("x: {}, y: {}", x, y);
    // }

    // println!("Число {} {}является палиндромом", input.trim(), if is_palindrome { "" } else { "не " });


}