fn main() {
    // let age = 20u8;
    // let big = 1_000_000_000_000i64;
    // println!("age is {}", age);
    // println!("big is {}", big);
    //
    //
    // let to_hex_num = 641900;
    // println!("{:#x}", to_hex_num); // to 16
    // println!("{:#o}", to_hex_num); // to 8
    // println!("{:#b}", to_hex_num); // to 8
    //
    // let check = 0xabcdef;
    // println!("check is {}", 2*2*2*2*2*2*2*2);

    let mut user_num1 = String::new();
    let mut user_num2 = String::new();

    // println!("enter u8 number 1!");
    std::io::stdin().read_line(&mut user_num1).expect("Not correct input");

    // println!("enter u8 number 2!");
    std::io::stdin().read_line(&mut user_num2).expect("Not correct input");

    let mut num1:i8 = user_num1.trim().parse().expect("Not u8 number");
    let mut num2:i8 = user_num2.trim().parse().expect("Not u8 number");


    println!("{:#b} + {:#b} = {:#b}", num1, num2, num1 + num2);
    println!("{:#o} + {:#o} = {:#o}", num1, num2, num1 + num2);
    println!("{:#x} + {:#x} = {:#x}", num1, num2, num1 + num2);
    println!("");
    println!("{:#b} - {:#b} = {:#b}", num1, num2, num1 - num2);
    println!("{:#o} - {:#o} = {:#o}", num1, num2, num1 - num2);
    println!("{:#x} - {:#x} = {:#x}", num1, num2, num1 - num2);
    println!("");
    println!("{:#b} * {:#b} = {:#b}", num1, num2, num1 * num2);
    println!("{:#o} * {:#o} = {:#o}", num1, num2, num1 * num2);
    println!("{:#x} * {:#x} = {:#x}", num1, num2, num1 * num2);
    println!("");
    println!("{:#b} / {:#b} = {:#b}", num1, num2, num1 / num2);
    println!("{:#o} / {:#o} = {:#o}", num1, num2, num1 / num2);
    println!("{:#x} / {:#x} = {:#x}", num1, num2, num1 / num2);
    println!("");
    println!("{:#b} % {:#b} = {:#b}", num1, num2, num1 % num2);
    println!("{:#o} % {:#o} = {:#o}", num1, num2, num1 % num2);
    println!("{:#x} % {:#x} = {:#x}", num1, num2, num1 % num2);
}