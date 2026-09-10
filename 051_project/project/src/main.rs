#[derive(Debug)]
struct Food {
    name: String
}

impl Food {
    fn new(name: String) -> Self {
        Self {
            name
        }
    }
}

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

    fn chef_special(&self, food: String) -> Option<Food> {
        match self.has_mice_infestation {
            true => {
                None
            }

            false => {
                Some(Food::new(food))
            }
        }
    }

    fn reservations_quantity(&self) -> Option<Food> {
        if self.reservations >= 12 {
            Some(Food::new(String::from("Strip Steak")))
        } else {
            Some(Food::new(String::from("Uni Sashimi")))
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
                    Ok(Food::new(String::from("Burger")))
                }
            }
        }
    }
}

fn main() {
    /* In the `main` function, create a `Restaurant` instance
with 11 reservations and a mice infestation.
 
Invoke the `chef_special` method and print out its return
value. It should be the None variant.
 
Invoke the `deliver_burger` method with an argument of "123
Elm Street" and print out its return value. It should be
the Err variant.
 
Create another `Restaurant` instance with 15 reservations
and no mice infestation.
 
Invoke the `chef_special` method and print out its return
 value. It should be the Some variant with a "Strip Steak".
 
Invoke the `deliver_burger` method with an argument of an
empty address. Print out its return value. It should be the
Err variant.
 
Invoke the `deliver_burger` method again with an argument
of a valid address. Print out its return value. It should
be the Ok variant nesting a Food struct with a `name` of
"Burger". */
    // só precisa terminar o main do project
    println!("Hello, world!");
}
