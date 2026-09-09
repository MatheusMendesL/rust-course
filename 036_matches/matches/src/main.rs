type Temperature = u32;

enum LaundryCicle {
    Cold(Temperature),
    Hot(Temperature),
    Delicate(String),
}

impl LaundryCicle {
    fn wash_laundry(&self) {
        match self {
            LaundryCicle::Cold(t) => {
                println!("Running the laundry with cold temperature, that is: {t}");
            }
            LaundryCicle::Hot(t) => {
                println!("Running the laundry with hot temperature, that is: {t}");
            }
            LaundryCicle::Delicate(fabric_type) => {
                println!("Running the laundry with a delicate cycle for {fabric_type}");
            }
        }
    }
}

fn main() {

    let laundry = LaundryCicle::Hot(32);
    laundry.wash_laundry();
}