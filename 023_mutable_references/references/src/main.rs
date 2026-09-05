fn main() {
    let mut hamburguer = String::from("hamburger");
    let a = &mut hamburguer;
    let b = a;
    println!("{b}");
    let city = create_city();
    println!("{city}");
}

fn create_city() -> String {
    let city = String::from("London");
    city

}