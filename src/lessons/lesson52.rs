fn main() {
    // let _arr:[i8; 2] = [8, 19];
    // let arr:[i32; 7] = [-1, 0, 1, 2, 30, 4, 500];
    // let mut user_num1 = String::new();
    // std::io::stdin().read_line(&mut user_num1).expect("Not correct input");
    // let index:usize = user_num1.trim().parse().expect("Not usize number");
    // println!("{}", arr[index]);


    // let mut input = std::io::stdin().lines();
    // let mut arr:[usize; 5] = [0; 5];
    //
    // for i in 0..5 {
    //
    // }
    //
    // // for val in arr {
    // //     ;
    // // }
    //
    // print!("{},", arr[arr[0]]);
    // print!("{},", arr[arr[1]]);
    // print!("{},", arr[arr[2]]);
    // print!("{},", arr[arr[3]]);
    // print!("{}",  arr[arr[4]]);

    let mut arr:[f64; 7] = [-621.5, 11.1, 2.0, -7.123, 0.125, 0.0, 0.000051789];
    let mut input = std::io::stdin().lines();

    let shuffle_index_start:usize = input.next().unwrap().unwrap().trim().parse().expect("Not correct input");
    let shuffle_index_end:usize = input.next().unwrap().unwrap().trim().parse().expect("Not correct input");
    // arr[index] = input.next().unwrap().unwrap().trim().parse().expect("Not correct input");

    let backup_value = arr[shuffle_index_start];
    arr[shuffle_index_start] = arr[shuffle_index_end];
    arr[shuffle_index_end] = backup_value;

    println!(
        "{:.9?}",
        arr
    );
}