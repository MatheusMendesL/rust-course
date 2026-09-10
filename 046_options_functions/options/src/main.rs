fn is_in_stock(in_system: bool, in_stock: bool) -> Option<bool> {
    if in_stock && in_system {
        Option::Some(true)
    } else if in_system {
        Option::Some(false)
    } else {
        Option::None
    }
}


fn main() {
    let availabity = is_in_stock(false, true);
    match availabity {
        Some(value) => {
            println!("Item is available: {value}");
        }
        None => {
            println!("Item is available: {}", availabity.unwrap_or(false));
        }
    }
}
