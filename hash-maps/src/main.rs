use std::collections::HashMap;

fn main() {
    let mut sauces_to_meals: HashMap<&str, Vec<&str>> = HashMap::from([
        ("Ketchup", vec!["French Fries", "Burgers", "Hot Dogs"]),
        ("Mayonnaise", vec!["Sandwiches", "Burgers", "Coleslaw"]),
    ]);

    sauces_to_meals.insert("Mustard", vec!["Hot dog", "Burgers", "Pretzels"]);
    match sauces_to_meals.remove("Mayonnaise") {
        Some(value) => println!("{:?}", value),
        None => println!("This did not work!"),
    }

    match sauces_to_meals.get("Mustard") {
        Some(value) => println!("{value:?}"),
        None => println!("This did not work!"),
    }

    sauces_to_meals
        .entry("Soy Sauce")
        .or_insert(vec!["sushi", "Dumplings"]);

    println!("{sauces_to_meals:#?}");
}
