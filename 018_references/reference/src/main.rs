fn main() {
    let value = 2;
    let borrow = &value;
    println!("{}", *borrow);

    let string = String::from("matheus");
    let reference_string = &string;
    println!("{reference_string}");

    let test = "string";
    println!("{}", test);
}
