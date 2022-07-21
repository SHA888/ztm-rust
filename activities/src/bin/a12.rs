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
  Black,
  Brown,
}

impl Color {
  fn print(&self) {
    match self {
      Color::Black => println!("black"),
      Color::Brown => println!("brown"),
    }
  }
}

struct Dimensions {
  width: f64,
  height: f64,
  depth: f64,
}

impl Dimensions {
    fn print(&self) {
      println!("width: {:?}", self.width); 
      println!("height: {:?}", self.height); 
      println!("depth: {:?}", self.depth)
    }
}

struct ShippingBox {
  dimensions: Dimensions,
  weight: f64,
  color: Color,
}

impl ShippingBox {
  fn new(dimensions: Dimensions, weight: f64, color: Color) -> Self {
    Self { dimensions, weight, color }
  }

  fn print(&self) {
    self.dimensions.print();
    println!("weight: {:?}", self.weight);
    self.color.print();
  }
}


fn main() {
  let small_dimensions = Dimensions {
    width: 3.0,
    height: 1.0,
    depth: 2.0,
  };

  let small_box = ShippingBox::new(small_dimensions, 7.5, Color::Brown);
  small_box.print();
}
