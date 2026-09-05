#[derive(Debug)]
struct Hamburguer {
    name: String,
    price: f64,
    is_on_stuff: bool
}

fn main() {
    let mut my_burguer = make_burguer(String::from("X tudo"), 25.99, true);
    eat_burguer(&mut my_burguer);
    println!("{:?}", my_burguer);
}

fn make_burguer(name: String, price: f64, is_on_stuff:bool) -> Hamburguer {
    Hamburguer {
        name,
        price,
        is_on_stuff
    }
}

fn eat_burguer (burguer: &mut Hamburguer) {
    println!("Eating my {} burguer", burguer.name);
    burguer.name = String::from("X bacon");
    burguer.price = 30.0;
    burguer.is_on_stuff = false;
}