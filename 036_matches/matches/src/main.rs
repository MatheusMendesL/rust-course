type Temperature = u32;

#[derive(Debug)]
enum LaundryCicle {
    Cold(Temperature),
    Hot(Temperature),
    Delicate(String),
}

impl LaundryCicle {
    fn wash_laundry(&self) {
        match self {
            LaundryCicle::Hot(t) => {
                println!("Running the laundry with hot temperature, that is: {t}");
            }
            /*
            aqui ele pefa um por 1 normal
            LaundryCicle::Hot(t) => {
                println!("Running the laundry with hot temperature, that is: {t}");
            }
            LaundryCicle::Delicate(fabric_type) => {
                println!("Running the laundry with a delicate cycle for {fabric_type}");
            }*/

            LaundryCicle::Cold(10) => {
                println!("Wow the temperature is 10");
            }

            // o q sobrar vem pra ca
            other_weather => {
                println!("Running the laundry with cold temperature, that is: {other_weather:?}");
            }
        }
    }
}

fn main() {

    let laundry = LaundryCicle::Cold(10);
    laundry.wash_laundry();
}