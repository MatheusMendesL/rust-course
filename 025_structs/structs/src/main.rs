fn main() {
   #[derive(Debug)]
   struct Hamburguer {
      name: String,
      price: f64,
      is_on_stuff: bool
   }

   let mut hamburguer = Hamburguer {
      name: String::from("x bacon"),
      price: 25.99,
      is_on_stuff: true
   };
   
   hamburguer.name = String::from("x everything");

   println!("My {} this night cost {}. It is {} that it was on the stuff",
   hamburguer.name, hamburguer.price, hamburguer.is_on_stuff);
}
