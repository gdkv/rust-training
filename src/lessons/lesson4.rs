fn main() {
    // let mut user_input = String::new();
    // println!("enter u8 number!");
    // std::io::stdin().read_line(&mut user_input).expect("Not correct input");
    // let num:u8 = user_input.trim().parse().expect("Not u8 number");
    // println!("user input is {num}!");
    // let num = 80 / 5 * 2;
    // println!("user input is {}!", -num);

    // let mut user_num1 = String::new();
    // let mut user_num2 = String::new();
    //
    // println!("enter i8 number 1!");
    // std::io::stdin().read_line(&mut user_num1).expect("Not correct input");
    //
    // println!("enter i8 number 2!");
    // std::io::stdin().read_line(&mut user_num2).expect("Not correct input");
    //
    // let num1:i8 = user_num1.trim().parse().expect("Not i8 number");
    // let num2:i8 = user_num2.trim().parse().expect("Not i8 number");
    //
    // println!("{}", num1 + num2);


    let mut user_num1 = String::new();
    let mut user_num2 = String::new();

    // println!("enter u8 number 1!");
    std::io::stdin().read_line(&mut user_num1).expect("Not correct input");

    // println!("enter u8 number 2!");
    std::io::stdin().read_line(&mut user_num2).expect("Not correct input");

    let num1:u8 = user_num1.trim().parse().expect("Not u8 number");
    let num2:u8 = user_num2.trim().parse().expect("Not u8 number");


    for x in [num1, num2] {
        for i in 1..=10 {
            println!("{} x {} = {}", x, i, x * i);
        }
        println!("");
    }
}