fn main() {
    let mut user_num1 = String::new();
    let mut user_num2 = String::new();

    std::io::stdin().read_line(&mut user_num1).expect("Not correct input");
    std::io::stdin().read_line(&mut user_num2).expect("Not correct input");

    let kg:f64 = user_num1.trim().parse().expect("Not f64 number");
    let lbs:f64 = user_num2.trim().parse().expect("Not f64 number");

     println!("{} kg = {:.3} lbs", kg, kg*2.205);
     println!("{} lbs = {:.3} kg", lbs, lbs*0.454);
     // println!("{:.0} * ({:.0}) = {:.0}", num1, num2, num1 * num2);
     // println!("{:.0} / ({:.0}) = {:.3}", num1, num2, num1 / num2);
     // println!("{:.0} % ({:.0}) = {:.3}", num1, num2, num1 % num2);
}