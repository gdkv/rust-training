use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка чтения ввода");
        let n: f64 = input.trim().parse().expect("Введите корректное число n");

        if n > 0.0 {
            println!("{}", n);
            break
        }

        input.clear(); // Очищаем буфер для следующего ввода
    }

}

