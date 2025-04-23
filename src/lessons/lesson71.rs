fn main () {
    let x:f64 = input();

    println!("{x:.3}");
}

fn input() -> f64 {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    return buf.trim().parse::<f64>().unwrap();
}