// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    RED,
    BLUE,
    GREEN,
}

struct Box {
    width: f64,
    height: f64,
    length: f64,
    weight: f64,
    color: Color,
}

impl Box {
    fn new_box(width: f64, height: f64, length: f64, weight: f64, color: Color) -> Self {
        Self {
            // It seems shorthand initialization is a thing.
            // It works if the parameter names and the struct field names is the same.
            width,
            height: height,
            length,
            weight: weight,
            color,
        }
    }

    fn print_characteristics(&self) {
        let color_txt = match self.color {
            Color::RED => "Red",
            Color::BLUE => "Blue",
            Color::GREEN => "Green",
        };
        println!(
            "Box: {{\n\twidth: {:?}\n\theight: {:?}\n\tlength: {:?}\n\tweight: {:?}\n\tcolor: {:?}\n}}",
            self.width, self.height, self.length, self.weight, color_txt
        );
    }
}

fn main() {
    let box_obj = Box::new_box(24.0, 24.0, 10.5, 96.5, Color::GREEN);
    box_obj.print_characteristics();
}
