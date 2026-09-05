fn main() {
    let apples: &str = "apples";
    print_my_value(apples);
    println!("{apples}");


    let apples2 = String::from("apples");
    print_my_value2(apples2);
    println!("{apples2}");
}

fn print_my_value(value: &str) {
    println!("{value}");
}

fn print_my_value2(value: String) {
    println!("{value}");
}
