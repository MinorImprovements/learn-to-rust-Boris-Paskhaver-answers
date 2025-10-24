fn main() {
    let num: i32 = 13_37;
    let _result: i16 = num as i16;
    let f_num: f32 = 3.1415673;
    println!("{:.3}", f_num);

    let with_milk: bool = true;
    let with_sugar: bool = true;
    let is_my_type_of_coffe: bool = with_sugar && with_sugar;
    let is_acceptable_coffee: bool = with_milk || with_sugar;

    let arr: [i8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    dbg!(arr);
    println!("{arr:?}");

    let toup: (i32, f32, bool, [i8; 8]) = (10, 9.9, true, arr);
    dbg!(toup);
    println!("{toup:?}");
}
