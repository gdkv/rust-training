fn main () {
    let mut input = std::io::stdin().lines();
    // let city = input.next().unwrap().unwrap();
    // let days:[u8;12] = [31,28,31,30,31,30,31,31,30,31,30,31];
    let month:usize = input.next().unwrap().unwrap().trim().parse().expect("Not u8 number");

    println!(
        "{}",
        if ([1,2,12].contains(&month)) {
            "Зима"
        } else if ([3,4,5].contains(&month)) {
            "Весна"
        } else if ([6,7,8].contains(&month)) {
            "Лето"
        } else {
            "Осень"
        }
    );
}