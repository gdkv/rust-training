fn main() {
    let mut input = std::io::stdin().lines();
    let x:u32 = input.next().unwrap().unwrap().trim().parse().expect("Not u8 number");
    let n:u8 = input.next().unwrap().unwrap().trim().parse().expect("Not u8 number");

    println!("{}", x >> n);
}
