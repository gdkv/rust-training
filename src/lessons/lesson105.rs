use std::io;

fn main() {
    let mut input = String::new();

    // Считываем первую строку (число n)
    // println!("Введите число n:");
    io::stdin().read_line(&mut input).expect("Ошибка чтения ввода");
    let n: u32 = input.trim().parse().expect("Введите корректное число n");

    input.clear(); // Очищаем буфер для следующего ввода

    // Считываем вторую строку (число k)
    // println!("Введите число k:");
    io::stdin().read_line(&mut input).expect("Ошибка чтения ввода");
    let k: u32 = input.trim().parse().expect("Введите корректное число k");


    // Получаем результат
    match remove_first_k_digits(n, k) {
        Ok(result) => println!("{}", result),
        Err(msg) => println!("{}", msg),
    }
}

/// Удаляет первые k цифр из числа n.
/// Возвращает:
/// - Ok(u32) — если осталось хотя бы одна цифра;
/// - Err(String) — если k >= количества цифр в n.
fn remove_first_k_digits(n: u32, k: u32) -> Result<u32, String> {
    // Преобразуем число в строку, чтобы посчитать цифры
    let s = n.to_string();
    let digit_count = s.len() as u32;

    if k >= digit_count {
        return Err("k >= n".to_string());
    }

    // Берем срез строки, начиная с k-й цифры
    let remaining = &s[k as usize..];

    // Преобразуем обратно в число
    remaining.parse::<u32>()
        .map_err(|e| format!("Ошибка преобразования: {}", e))
}