// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn number_add(a: i32, b: i32) -> i32 {
    a + b
}

fn display(a: i32) {
    println!("{:?}", a);
}

fn main() {
    let added_value = number_add(1, 2);

    display(added_value);
}
