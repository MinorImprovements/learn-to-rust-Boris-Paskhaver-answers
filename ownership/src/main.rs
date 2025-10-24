fn main() {
    //let is_concert: bool = true;
    //let is_event: bool = is_concert;
    //rust will move ownership to is_event
    //println!("{}", is_concert);
    //println!("{}", is_event);
    //rust gave ownership to both

    //let sushi: &str = "Salmon";
    //let dinner: &str = sushi;
    //rust will move ownership to dinner
    //println!("{sushi}");
    //println!("{dinner}");
    //rust gave ownership to both
    //let sushi: String = String::from("Salmon");
    //let dinner: String = sushi;
    //rust will move ownership to dinner
    //println!("{sushi}");
    //println!("{dinner}");
    // dinner has ownership because it is from the heap
    //println!("{:?}", eat_meal(sushi));

    let mut trip: String = start_trip();
    visit_philadelphia(&mut trip);
    visit_new_york(&mut trip);
    visit_boston(&mut trip);
    show_itinerary(&trip);
}

fn start_trip() -> String {
    String::from("The plan is...")
}

fn visit_philadelphia(trip: &mut String) {
    trip.push_str("Philadelphia")
}

fn visit_new_york(trip: &mut String) {
    trip.push_str(" and New York")
}

fn visit_boston(trip: &mut String) {
    trip.push_str(" and Boston")
}

fn show_itinerary(trip: &String) {
    println!("{}", trip);
}
//fn eat_meal(mut meal: String) {
//    meal.clear()
//}
