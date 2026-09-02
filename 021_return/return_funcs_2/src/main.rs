fn main() {
    let mut current_meal = String::new();
    add_salt(&mut current_meal);
    add_rice(&mut current_meal);
    show_the_meal(&current_meal);
}

fn add_salt(meal: &mut String) {
    meal.push_str("add salt");
}

fn add_rice(meal: &mut String) {
    meal.push_str(" add rice");
}

fn show_the_meal(meal: &String){
    println!("meal steps: {meal}");
}