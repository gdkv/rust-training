// use std::io;

fn main() {
    // let (name, x) = ("dima", 33);
    // let mut age = 44;
    // age = 344;
    // let str = hello();
    // println!("{str} my name is {name} and my age is {}", age.to_string());

    // lesson33();
    lesson34();
}

// fn hello() -> String {
//     return "Hello, world TEST TEST! 🤫❌🔥 ".to_string();
// }

// fn lesson33() {
//     let (a, b, c, d, e, f) = (5, 10, 15, 20, 25, 30);
//
//     // "a = 5, b = 10, c = 15, d = 20, e = 25, f = 30"
//     // "a = 5, b = 10, c = 15, d = 20, e = 25, f = 30
//     println!("a = {3}, b = {}, c = {5}, d = {}, e = {4}, f = {}", b, d, f, a, e, c);
//     // println!("b, d, f, a, e, c")
//     println!("b, d, f, a, e, c");
// }

// fn lesson34() {
//     let mut user_input = String::new();
//     println!("Enter user name:");
//     let result = io::stdin().read_line(&mut user_input);
//     if result.is_ok() {
//         println!("User input: {user_input}");
//         let in_num: i32 = user_input
//             .trim()
//             .parse()
//             .expect("Please type a number!");
//         println!("In number + 1 = {}", in_num + 1);
//     } else {
//         println!("error");
//     }
// }

// pub fn lesson34() {
//     let mut total = String::new();
//     for _ in 0..4 {
//         let mut user_input = String::new();
//         std::io::stdin()
//             .read_line(&mut user_input)
//             .expect("Failed to read line");
//         total.insert_str(0, &user_input);
//     }
//     print!("{total}");
// }

// pub fn lesson34() {
//     let mut user_input = String::new();
//     let mut user_num: i32 = 0;
//
//     std::io::stdin()
//         .read_line(&mut user_input)
//         .expect("lol kek!");
//     user_num = user_input.trim().parse().unwrap();
//     println!("{user_num}");
// }