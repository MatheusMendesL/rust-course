fn color_to_number_if(color: &str) -> i32 {
    if color == "red" {
        return 1;
    } else if color == "green"  {
        return 2;
    } else if color == "blue" {
        return 3;
    } else {
        return 0;
    }
}

fn color_to_number_match(color: &str) -> i32 {
    match color {
        "red" => return 1,
        "green" => return 2,
        "blue" => return 3,
        _ => return 0
    }
}

fn factorial(number :i32) -> i32 {
    let mut result = 1;
    let mut sequence = number;
    loop {
        if sequence == 0 {
            return result;
        }

        result = result * sequence;
        sequence -= 1;
    }
}

fn factorial_r(number: i32, result: i32) {
    if number == 1 {
        println!("{}", result);
        return;
    }

    factorial_r(number - 1, result * number);
}

fn main() {
    println!("{}", color_to_number_if("orange"));
    println!("{}", color_to_number_match("blue"));
    println!("{}", factorial(5));
    factorial_r(4, 1);
}
