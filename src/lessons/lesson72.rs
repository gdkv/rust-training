fn main () {
    let mut input = std::io::stdin().lines();
    // let city = input.next().unwrap().unwrap();
    // let nums:[u8;3] = [
    let age:u8 = input.next().unwrap().unwrap().trim().parse().expect("Not u8 number");
    let sys:u8 = input.next().unwrap().unwrap().trim().parse().expect("Not u8 number");
    let dia:u8 = input.next().unwrap().unwrap().trim().parse().expect("Not u8 number");

    let sys_check:bool = false;
    let dia_check:bool = false;
    // ];

    // for num in nums {
    //     println!(
    //         "{} в город {} вход {}",
    //         num,
    //         city.trim(),
    //         if (city.trim() == "Четный" && num % 2 == 0) || (city.trim() == "Нечетный" && num % 2 != 0) { "разрешен" } else { "запрещен" }
    //     );
    // }

    if age >= 18 && age <= 39 {
        let sys_check:bool = sys >= 90 && sys <= 139;
        let dia_check:bool = dia >= 60 && dia <= 89;


    } else if age >= 40 && age <= 59 {
        let sys_check:bool = sys >= 91 && sys <= 149;
        let dia_check:bool = dia >= 61 && dia <= 91;
    } else {
        let sys_check:bool = sys >= 91 && sys <= 159;
        let dia_check:bool = dia >= 61 && dia <= 91;
    }

    println!(
        //         "{} в город {} вход {}",
        //         num,
        //         city.trim(),
        //         if (city.trim() == "Четный" && num % 2 == 0) || (city.trim() == "Нечетный" && num % 2 != 0) { "разрешен" } else { "запрещен" }
    );

    fn check_dia_sys (is_sys:bool, is_dia:bool) -> Sting {
        if is_sys && is_dia {
            return "Систолическое и Диастолическое АД в норме";
        } else if sys_check {
            return "Систолическое АД в норме\n";
        } else if dia_check {
            return "Диастолическое АД в норме\n";
        } else {

        }
    }
}