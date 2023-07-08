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

impl Color {
    fn string(&self) -> &str {
        match self {
            Color::RED => "Red",
            Color::BLUE => "Blue",
            Color::GREEN => "Green",
        }
    }
}

struct Dimension {
    width: f64,
    height: f64,
    length: f64,
}

struct Box {
    dimensions: Dimension,
    weight: f64,
    color: Color,
}

impl Box {
    fn new(width: f64, height: f64, length: f64, weight: f64, color: Color) -> Self {
        let dimensions = Dimension {
            width,
            height,
            length,
        };

        Self {
            // It seems shorthand initialization is a thing.
            // It works if the parameter names and the struct field names is the same.
            dimensions: dimensions,
            weight: weight,
            color,
        }
    }

    fn print_characteristics(&self) {
        println!(
            "Box: {{\n\twidth: {:?}\n\theight: {:?}\n\tlength: {:?}\n\tweight: {:?}\n\tcolor: {:?}\n}}",
            self.dimensions.width, self.dimensions.height, self.dimensions.length, self.weight, self.color.string()
        );
    }
}

fn main() {
    let box_obj = Box::new(24.0, 24.0, 10.5, 96.5, Color::GREEN);
    box_obj.print_characteristics();
}
