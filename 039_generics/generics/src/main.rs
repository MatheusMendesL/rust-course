#[derive(Debug)]
struct sandwich {}


fn main() {
    println!("{}", identity::<i32>(5));
    println!("{}", identity::<f64>(3.14));
    println!("{}", identity::<&str>("hello"));
    println!("{:?}", identity::<sandwich>(sandwich {}));
}

fn identity<T>(value: T) -> T {
    value
}
