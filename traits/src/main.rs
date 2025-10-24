use std::fmt::{self, Debug, Display, Formatter};

fn main() {
    let mut latte: Coffee<&str> = Coffee::new("Latte", Milk::Oat, 16);
    println!("{:?}", latte);
    latte.consume();
    println!("{:?}", latte);

    let cappuccino: Coffee<String> = Coffee::new("Cappuccino".to_string(), Milk::Almond, 8);
    cappuccino.stats();

    let pepsi: Soda = Soda::new(100, 5.75, "Cherry".to_string());
    println!("{pepsi}");

    let mut coke = pepsi.clone();
    println!("{}", coke == pepsi);

    coke.consume();
    println!("{:?}", coke);
}

trait Drinkable {
    fn consume(&mut self);
    fn get_data(&self) -> String;
    fn stats(&self) {
        println!("{}", self.get_data());
    }
} //Drinkable Trait

#[derive(Debug)]
enum Milk {
    Whole,
    Oat,
    Almond,
}

struct Coffee<T> {
    kind: T,
    milk: Milk,
    ounces: u32,
}

impl<T> Coffee<T> {
    fn new(kind: T, milk: Milk, ounces: u32) -> Self {
        Self { kind, milk, ounces }
    }
}

impl<T: Debug> Debug for Coffee<T> {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        format
            .debug_struct("DIRTY WATER")
            .field("kind", &self.kind)
            .field("milk", &self.milk)
            .field("ounces", &self.ounces)
            .finish()
    }
}

impl<T: Display> Drinkable for Coffee<T> {
    fn consume(&mut self) {
        self.ounces = 0;
    }

    fn get_data(&self) -> String {
        format!("A delicous {} ounce {}", self.ounces, self.kind)
    }
}

#[derive(Debug)]
struct Soda {
    calories: u32,
    price: f64,
    flavor: String,
    percentage: u32,
}

impl Soda {
    fn new(calories: u32, price: f64, flavor: String) -> Self {
        Self {
            calories,
            price,
            flavor,
            percentage: 100,
        }
    }
}

impl Drinkable for Soda {
    fn consume(&mut self) {
        self.percentage = 0;
    }

    fn get_data(&self) -> String {
        format!("Flavor: {}, Calories : {}", self.flavor, self.calories)
    }
}

impl Display for Soda {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        write!(format, "** {} Soda **", self.flavor)
    }
}

impl Clone for Soda {
    fn clone(&self) -> Self {
        Self {
            calories: self.calories,
            price: self.price,
            flavor: self.flavor.clone(),
            percentage: self.percentage,
        }
    }
}

impl PartialEq for Soda {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Eq for Soda {}
