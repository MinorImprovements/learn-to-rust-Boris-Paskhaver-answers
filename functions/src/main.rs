fn main() {
    apply_to_jobs(20, "tech 3");
    let result = is_even(3);
    println!("{result}");
    let result = is_even(2);
    println!("{result}");
    let alpha = alphabets("xavier");
    println!("{alpha:?}");
    let alpha = alphabets("zebra");
    println!("{alpha:?}");
}

fn apply_to_jobs(number: i32, title: &str) {
    println!("I'm applytng to {number} {title} jobs.")
}

fn is_even(number: i32) -> bool {
    number % 2 == 0
}

fn alphabets(text: &str) -> (bool, bool) {
    let first_check = text.contains('a');
    let second_check = text.contains('z');
    (first_check, second_check)
}
