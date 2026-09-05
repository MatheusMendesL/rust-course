fn main() {
    let mut car = String::from("hello");
    let ref1 = &mut car;
    add_word(", world", ref1);
    let ref2 = &car;
    println!("{} ", ref2);
}

fn add_word(word: &str, refer: &mut String){
    refer.push_str(word);
}