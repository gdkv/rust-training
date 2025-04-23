fn main() {
    // let mut input = std::io::stdin().lines();
    // let user_num:u16 = input.next().unwrap().unwrap().trim().parse().expect("Not correct input");
    //
    // let first_num = user_num/1000;
    // let second_num = user_num%1000/100;
    // let third_num = user_num%100/10;
    // let forth_num = user_num%10;
    //
    // print!("Последняя цифра: {forth_num}\n");
    // print!("Третья цифра: {third_num}\n");
    // print!("Вторая цифра: {second_num}\n");
    // print!("Первая цифра: {first_num}");

    // let mut user_str:String = String::new();
    // std::io::stdin().read_line(&mut user_str).expect("lala");
    // user_str = String::from(user_str.trim());
    // let user_num: f64 = user_str.parse().expect("Not u8 number");
    //
    // print!("{} км = {:.3} миль\n", user_str.trim(), user_num * 0.621371);
    // print!("{} км = {:.3} ярдов\n", user_str.trim(), user_num * 1093.61);
    // print!("{} км = {:.3} футов\n", user_str.trim(), user_num * 3280.84);
    // print!("{} км = {:.3} дюймов", user_str.trim(), user_num * 39370.1);

    // let mut user_str:String = String::new();
    // std::io::stdin().read_line(&mut user_str).expect("lala");
    // let sec_count: u64 = user_str.trim().parse().expect("Not u64 number");
    // println!("{} сек = {} час {} минут {} секунд", sec_count, sec_count/3600, (sec_count%3600)/60, sec_count%60);

    //
    // let mut input = std::io::stdin().lines();
    // let questions:S = input.next().unwrap().unwrap().trim().parse().expect("Not correct input");
    // dbg!(ready);
    // println!("Вероятность попадания выученного вопроса: {}", (1.00 - ready/questions));
    // todo!("Write a function to reverse");
    //
    // let my_temporary_debug_variable = question.split(' ').collect::<Vec<_>>();
    // println!("{:?}", my_temporary_debug_variable);

    let mut input = std::io::stdin().lines();

    let num:f64 = input.next().unwrap().unwrap().trim().parse().expect("Not correct input");

    if num > 0.0 {
        println!("Число {num:.1} является положительным");
    } else {
        println!("Число {num:.1} является отрицательным");
    }

}