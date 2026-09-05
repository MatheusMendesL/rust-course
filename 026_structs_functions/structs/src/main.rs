#[derive(Debug)]
struct Hamburguer {
    name: String,
    price: f64,
    is_on_stuff: bool
}

fn main() {
    let my_burguer = make_burguer(String::from("X tudo"), 25.99, true);
    let another_burguer = Hamburguer {
       name: my_burguer.name.clone(),
        ..my_burguer
    };
    println!("{:?}", my_burguer);
    println!("{:?}", another_burguer);
}

fn make_burguer(name: String, price: f64, is_on_stuff:bool) -> Hamburguer {
    Hamburguer {
        name,
        price,
        is_on_stuff
    }
}
