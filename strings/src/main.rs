use std::io;

fn make_money(money: &mut String) {
    money.push_str("$$$")
}

fn trim_and_capitalize(phrase: &str) -> String {
    phrase.trim().to_uppercase()
}

fn elements(new_collection: &str) -> Vec<&str> {
    new_collection.split("!").collect()
}

fn get_identity() -> String {
    let mut first_name: String = String::new();
    let mut last_name: String = String::new();

    println!("Enter your first name:");
    io::stdin()
        .read_line(&mut first_name)
        .expect("Falied to retrieve first name...");
    println!("Enter your last name:");
    io::stdin()
        .read_line(&mut last_name)
        .expect("Falied to retrieve last name...");
    format!("{} {}", first_name.trim(), last_name.trim())
}

fn main() {
    let mut my_money: String = String::from("5000");
    make_money(&mut my_money);
    println!("{my_money}");

    let too_much_whitespace: &str = "    thats alot of whitespace     ";
    let cleaned_phrase: String = trim_and_capitalize(too_much_whitespace);
    println!("{cleaned_phrase}");

    let metal_string: &str = "Gold!Silver!Plantium";
    let metals: Vec<&str> = elements(metal_string);
    println!("{:#?}", metals);

    let fullname: String = get_identity();
    println!("{fullname}");
}
