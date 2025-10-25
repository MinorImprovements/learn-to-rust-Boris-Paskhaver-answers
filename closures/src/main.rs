#[derive(Debug)]
struct SuperMartketItem {
    name : String,
    price : f64
}

#[derive(Debug)]
struct ShoppingCart {
    items : Vec<SuperMartketItem>
}

impl ShoppingCart {
    fn traverse_items<F : FnMut(&mut SuperMartketItem)>(&mut self, mut operation : F){
        let mut current_element = 0;
        while current_element < self.items.len() {
            operation(&mut self.items[current_element]);
            current_element += 1;
        }
    }

    fn checkout<F : FnOnce(ShoppingCart)>(self, operation : F) {
        operation(self);
    }
}

fn main() {
    let mut cart_items : ShoppingCart = ShoppingCart { items: 
            ( Vec::<SuperMartketItem>::from([
                SuperMartketItem {
                name : "APPLE".to_string(),
                price: 3.99
            },
            SuperMartketItem {
                name : "BANANA".to_string(),
                price : 2.99
            }
        ]) 
    )};

    cart_items.traverse_items(|item : &mut SuperMartketItem| item.price *= 0.85 );
    cart_items.traverse_items(|item : &mut SuperMartketItem| item.name = item.name.to_lowercase());
    
    let mut total_price = 0.0;
    cart_items.checkout(|mut cart : ShoppingCart| {
        println!("{cart:#.2?}");
        cart.traverse_items(|items: &mut SuperMartketItem| total_price += items.price);
        println!("Total Price: ${total_price:.2}");
    });
}
