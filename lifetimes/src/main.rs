fn main() {
    let vec1 = double_the_length(&vec![1, 2, 3]);
    println!("{vec1}");
    println!("{:?}", last_two(&vec![1, 2, 3]));
    let return_text = first_five("refrigerator", "Hello");
    println!("{return_text}");
    let return_content = find_string_that_has_content("programming", "dining", "gram");
    println!("{return_content}");
}

/*
* This function does not need lifetimes because it is not returning a reference,
* it is returning a new type. There is also only one peram which allows the compiler to
* infer that the lifetime of the return value is tied to the lifetime of the param
*/
fn double_the_length<T>(vector: &[T]) -> usize {
    vector.len() * 2
}

fn last_two<T>(slice: &[T]) -> &[T] {
    &slice[(slice.len() - 2)..]
}

/*
* yes, this function will need lifetime annotations because there are multiple reference perams
*/
fn first_five<'a>(text: &'a str, announcement: &str) -> &'a str {
    println!("{announcement}");
    &text[..5]
}

/*
* yes, Too many params and a returned reference
*/

fn find_string_that_has_content<'a>(first: &'a str, second: &'a str, target: &'a str) -> &'a str {
    if first.contains(target) {
        return first;
    }
    second
}
