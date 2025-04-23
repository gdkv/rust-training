fn main() {
    // let tup = ("До", "деструктуризации", 3, 2, 1, 0, "пуск!");
    // let (str1, str2, num1, num2, num3, num4, str3) = tup;
    // println!(
    //     "{}, {}, {}, {}, {}, {}, {}",
    //     str1, str2, num1, num2, num3, num4, str3
    // );

    let mut tup:(f32, f32, f32, f32, f32, f32) = (10.0, 5.0, -2.0, 100.0, 2000.0, 0.0);

    let mut user_num1:String = String::new();
    let mut user_num2:String = String::new();
    let mut user_num3:String = String::new();
    let mut user_num4:String = String::new();
    let mut user_num5:String = String::new();

    std::io::stdin().read_line(&mut user_num1).expect("Not correct input");
    std::io::stdin().read_line(&mut user_num2).expect("Not correct input");
    std::io::stdin().read_line(&mut user_num3).expect("Not correct input");
    std::io::stdin().read_line(&mut user_num4).expect("Not correct input");
    std::io::stdin().read_line(&mut user_num5).expect("Not correct input");

    let num1:f32 = user_num1.trim().parse().expect("Not f32 number");
    let num2:f32 = user_num2.trim().parse().expect("Not f32 number");
    let num3:f32 = user_num3.trim().parse().expect("Not f32 number");
    let num4:f32 = user_num4.trim().parse().expect("Not f32 number");
    let num5:f32 = user_num5.trim().parse().expect("Not f32 number");

    tup.0 = num1 as f32;
    tup.1 = num2 as f32;
    tup.2 = num3 as f32;
    tup.3 = num4 as f32;
    tup.4 = num5 as f32;

    println!(
        "{}, {}, {}, {}, {}, {}",
        tup.0.trunc(),
        tup.1.trunc(),
        tup.2.trunc(),
        tup.3.trunc(),
        tup.4.trunc(),
        tup.5.trunc()
    );
}