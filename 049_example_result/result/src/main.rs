fn main() {
    let text = "50";
    let number = text.parse::<i32>();
    println!("{:?}", number);


    let text = "text";
    let number = text.parse::<i32>();
    println!("{:?}", number);
}
