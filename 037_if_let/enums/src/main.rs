type Kind = String;

enum Milk {
    LowFat(i32),
    Whole,
    NonDairy(Kind),
}

fn main() {
    let my_beverage = Milk::NonDairy(String::from("Oat"));

    if let Milk::NonDairy(kind) = my_beverage {
        println!("You have a NonDairy milk, the percent is {kind}");
    }
}
