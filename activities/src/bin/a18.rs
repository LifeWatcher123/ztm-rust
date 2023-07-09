// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

#[derive(Debug)]
struct Adult {
    name: String,
    age: i32,
}

impl Adult {
    // Crash course answer instead used a &str instead of String as an argument here.
    // That would reduce the usage of to_owned when calling the new function.
    fn new(name: String, age: i32) -> Result<Self, String> {
        if age >= 21 {
            return Result::Ok(Self { name, age });
        }
        return Result::Err("Failed to create Adult (Reason: age is >= 21)".to_owned());
    }
}

fn main() {
    let results_adult = vec![
        Adult::new("Dan".to_owned(), 24),
        Adult::new("Christopher".to_owned(), 20),
    ];

    for result in results_adult {
        match result {
            Result::Ok(adult) => println!("Adult {:?} created. \n{:?}", adult.name, adult),
            Result::Err(errmsg) => println!("{:?}", errmsg),
        }
    }
}
