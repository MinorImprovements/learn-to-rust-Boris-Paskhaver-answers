#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Flight {
        Flight {
            origin,
            destination,
            price,
            passengers,
        }
    }
}

impl Flight {
    fn new_destination(&mut self, dif_destination: String) -> &mut Flight {
        self.destination = dif_destination;
        self
    }

    fn increase_price(&mut self) -> &mut Flight {
        self.price *= 1.20;
        self
    }

    fn itinerary(&self) {
        println!("{:?} -> {:?}", self.origin, self.destination)
    }
}

fn main() {
    let mut flight = Flight::new(
        String::from("Washington"),
        String::from("Minnesota"),
        450.69,
        180,
    );

    flight.new_destination(String::from("Arizona"));
    flight.increase_price();
    flight.itinerary();

    println!("{:?}", flight);
    println!("{:#?}", flight);

    let mut flight2 = Flight {
        origin: String::from("USA"),
        destination: String::from("Japan"),
        ..flight
    };

    println!("{:#?}", flight2)
}
