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
#![allow(dead_code)]
fn main() {
    let box_item = Box::new(Dimensions::new(7.0, 8.0, 4.0), 345, BoxColour::GREEN);
    box_item.print();
}

#[derive(Debug)]
struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn new(width: f64, height: f64, depth: f64) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }
    fn print(&self) {
        println!("width {:?}", self.width);
        println!("height {:?}", self.height);
        println!("depth {:?}", self.depth);
    }
}

#[derive(Debug)]

struct Box {
    dimension: Dimensions,
    weight: u32,
    colour: BoxColour,
}

impl Box {
    fn new(dimension: Dimensions, weight: u32, colour: BoxColour) -> Self {
        Self {
            dimension,
            weight,
            colour,
        }
    }

    fn print(&self) {
        self.colour.print();
        self.dimension.print();
        println!("weight {:?}", self.weight);
    }
}

#[derive(Debug)]

enum BoxColour {
    RED,
    BLUE,
    GREEN,
}

impl BoxColour {
    fn print(&self) {
        match self {
            Self::GREEN => println!("green"),
            Self::BLUE => println!("blue"),
            Self::RED => println!("red"),
        }
    }
}
