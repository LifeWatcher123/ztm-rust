// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    fav_color: String,
}

impl Person {
    fn print_info(&self) {
        println!(
            "Name: {:?}; Favorite Color/s: {:?}",
            self.name, self.fav_color
        );
    }
}

fn main() {
    let people = vec![
        Person {
            age: 8,
            name: String::from("Alex"),
            fav_color: String::from("Red"),
        },
        Person {
            age: 5,
            name: String::from("Bryan"),
            fav_color: String::from("Yellow"),
        },
        Person {
            age: 25,
            name: String::from("Audra"),
            fav_color: String::from("Orange"),
        },
    ];

    for person in people {
        if person.age <= 10 {
            person.print_info();
        }
    }
}
