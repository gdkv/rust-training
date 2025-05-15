fn main () {
    let mut input = std::io::stdin().lines();
    let min_input:usize = input.next().unwrap().unwrap().trim().parse().expect("Not u8 number");
    let max_input:usize = input.next().unwrap().unwrap().trim().parse().expect("Not u8 number");
//
//     println!(
//         "{}",
//         if [1,2,12].contains(&month) {
//             "Зима"
//         } else if [3,4,5].contains(&month) {
//             "Весна"
//         } else if [6,7,8].contains(&month) {
//             "Лето"
//         } else {
//             "Осень"
//         }
//     );

    // let tup = (1, 3.14, -12.3, -50, 100, 250, -4, 7.6);
    let array:[i32;8] = [3, 1, 0, -5, -1, 300, 4, 2];
    let mut min:i32 = 0;
    let mut max:i32 = 0;

    for num in array {
        if num > max {
            max = num;
        }

        if num < min {
            min = num;
        }
    }

    println!("Считанный мин.индекс {}", if array[min_input] == min {"корректный"} else {"некорректный"});
    println!("Считанный макс.индекс {}", if array[max_input] == max {"корректный"} else {"некорректный"});
}