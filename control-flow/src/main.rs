fn main() {
    fn color_to_number(color: &str) -> i32 {
        match color {
            "red" => 1,
            "green" => 2,
            "blue" => 3,
            _ => 0,
        }
    }

    fn loop_factorial(mut number: i32) -> i32 {
        let mut count = number;
        while count > 1 {
            number *= count - 1;
            count -= 1;
        }
        number
    }

    fn recursion_factorial(number: i32) -> i32 {
        if number == 1 {
            1
        } else {
            number * recursion_factorial(number - 1)
        }
    }

    let color: &str = "red";
    println!("{}", color_to_number(color));
    println!("{}", loop_factorial(5));
    println!("{}", recursion_factorial(5));
} //End of main function
