// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn coodinate(x: i32, y: i32, z: i32) -> (i32, i32, i32) {
    (x, y, z)
}

fn main() {
    // Looks like, similar to other languages, that "_" means disregard.
    // Seems similar to the "everything else" notation of the match block.
    let (_, y, _) = coodinate(12, 5, 15);

    if y > 5 {
        println!("Y-value is greater than 5");
    } else if y < 5 {
        println!("Y-value is less than 5");
    } else {
        println!("Y-value is equal to 5");
    }
}
