fn main() {
    let mut input = std::io::stdin().lines();
    let card_number = input.next().unwrap().unwrap();
    let mut sum: u32 = 0;
    let mut sum_not_plural: u32 = 0;
    let mut more_then_four: u32 = 0;
    let mut card_str_number: String = String::from("");

    for (i, c) in card_number.chars().enumerate() {
        if i % 4 == 0 {
            card_str_number += String::from(" ").as_str();
        }
        let number: u8 = c as u8 - 0x30;
        sum += number as u32;
        if (i + 1) % 2 != 0 {
            sum_not_plural += number as u32;
            if number > 4 {
                more_then_four += 1;
            }
        }
        card_str_number += String::from(c).as_str();
    }

    if (sum + sum_not_plural + more_then_four) % 10 == 0 {
        println!("Карта с номером {} действительна", card_str_number.trim());
    } else {
        println!("Карты с номером {} не существует", card_str_number.trim());
    }
}
