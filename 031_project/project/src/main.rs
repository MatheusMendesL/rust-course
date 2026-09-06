#[derive(Debug)]
enum FlightStatus {
    Scheduled,
    Boarding,
    Departed,
    Arrived,
    Cancelled,
}

#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
    status: FlightStatus,
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32, status: FlightStatus) -> Self {
        Self {
            origin,
            destination,
            price,
            passengers,
            status
        }
    }

    fn change_destination(&mut self, new_destination: String) -> &mut Self {
        self.destination = new_destination;
        self
    }

    fn increase_price(&mut self) -> &mut Self {
        self.price = self.price * 1.20;
        self
    }

    fn itinerary(&self) {
        println!(
            "The origin is {} and the destination is: {}",
            self.origin, self.destination
        );
    }
}

fn main() {
    let mut flight = Flight::new(
        String::from("São Paulo"),
        String::from("Rio de janeiro"),
        100.00,
        167,
        FlightStatus::Arrived
    );

    flight
        .change_destination(String::from("Madrid"))
        .increase_price()
        .itinerary();

    let flight2 = Flight {
        origin: String::from("Brasília"),
        destination: String::from("Maranhão"),
        ..flight
    };

    println!("{:?}", flight2);
}
