fn main() {
    let registrations = [true, false, true];
    let index0 = registrations[0];
    println!("{index0} and {registrations:?}");

    let languages = [String::from("Rust"), String::from("Golang")];
    let first = &languages[1];
    println!("{first} and {languages:?}");
}
