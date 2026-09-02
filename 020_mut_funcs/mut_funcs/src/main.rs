/* fn main() {
    let comida: String = String::from("feijoada");
    println!("{}", add_meat(comida));
}

fn add_meat(mut meat: String) -> String{
    meat.push_str(" e Linguiça ");
    return meat;
} */

fn main(){
    let burger = make_hambuger();
    println!("My hamburguer is a {burger}");
}

fn make_hambuger() -> String{
    return String::from("x bacon");
}