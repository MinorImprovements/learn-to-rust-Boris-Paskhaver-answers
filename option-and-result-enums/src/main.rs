use std::collections::hash_map;

#[derive(Debug)]
struct Food {
    name: String,
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        let famous_dish = match self.reservations {
            x if x < 12_u32 => String::from("Uni Sashimi"),
            x if x >= 12_u32 => String::from("Strip Steak"),
            _ => panic!("panic within the chef_special method!"),
        };

        match self.has_mice_infestation {
            true => None,
            false => Some(Food { name: famous_dish }),
        }
    }

    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            return Err(String::from("Sorry, we have a mice problem"));
        }
        if address.is_empty() {
            return Err(String::from("No delivery address specified"));
        }

        Ok(Food {
            name: String::from("burger"),
        })
    }
}

fn main() {
    let olives = Restaurant {
        reservations: 11,
        has_mice_infestation: true,
    };

    let pearls = Restaurant {
        reservations: 15,
        has_mice_infestation: false,
    };

    println!("{:?}", olives.chef_special());
    println!("{:?}", olives.deliver_burger("123 Elm Street"));

    println!("{:?}", pearls.chef_special());
    println!("{:?}", pearls.deliver_burger(""));
    println!("{:?}", pearls.deliver_burger("123 Elm Street"));
}
