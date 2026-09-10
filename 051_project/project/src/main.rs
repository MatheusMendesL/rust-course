#[derive(Debug)]
#[allow(dead_code)]
struct Food {
    name: String,
    price: f64,
    is_on_stock: bool
}

impl Food {
    fn new(name: String, price: f64, is_on_stock: bool) -> Self {
        Self {
            name,
            price,
            is_on_stock
        }
    }

    fn verify_stock(self) -> Option<Self> {
        if self.is_on_stock {
            return Some(self)
        } else {
            return None;
        }
    }
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool
}

impl Restaurant {
    fn new(reservations: u32, has_mice_infestation: bool) -> Self {
        Self {
            reservations,
            has_mice_infestation
        }
    }

    fn chef_special(&self) -> Option<Food> {
        match self.has_mice_infestation {
            true => {
                None
            }

            false => {
                self.reservations_quantity().and_then(|food| food.verify_stock())
            }
        }
    }

    fn reservations_quantity(&self) -> Option<Food> {
        if self.reservations >= 12 {
            Some(Food::new(String::from("Strip Steak"), 90.00, true))
        } else {
            Some(Food::new(String::from("Uni Sashimi"), 32.50, true))
        }
    }


    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        match self.has_mice_infestation {
            true => {
                Err(String::from("Sorry, we have a mice problem"))
            }

            false => {
                if address.is_empty(){
                    Err(String::from("No delivery address specified"))
                } else {
                    Ok(Food::new(String::from("Burger"), 40.99, true))
                }
            }
        }
    }
}

fn main() {
    let rest = Restaurant::new(11, true);
    println!("{:#?}", rest.chef_special());
    println!("{:#?}", rest.deliver_burger("123 Elm Street"));

    let rest2 = Restaurant::new(15, false);
    println!("{:#?}", rest2.chef_special());
    println!("{:#?}", rest2.deliver_burger(""));
    println!("{:#?}", rest2.deliver_burger("123 Elm Street"));

}
