fn main () {
    let x:u32 = input();
    let y:u32 = input();

    let coins:[u32;4] = [1,2,5,10];
    let money:[u32;9] = [5,10,50,100,200,500,1000,2000,5000];

    if coins.contains(&x) {
        println!("Принята монета номинала {x}");
    } else {
        println!("Монеты такого номинала не принимаются");
    }

    if money.contains(&y) {
        println!("Принята купюра номинала {y}");
    } else {
        println!("Купюры такого номинала не принимаются");
    }
}

fn input() -> u32 {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    return buf.trim().parse::<u32>().unwrap();
}