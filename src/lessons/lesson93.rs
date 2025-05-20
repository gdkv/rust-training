fn main() {
    let byte = 0x00;
    let mask = 0x01;

    println!("{:08b} :byte", byte);
    println!("{:08b} :mask", mask);
    println!("{:08b} :byte | mask", byte | mask);
}
