// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
  Cola,
  Chocolate,
  Strawberry,
  Vanilla,
}

struct Drink {
  flavor: Flavor,
  fluid_oz: f64,
}

fn print_drink(drink: Drink) {
  match drink.flavor {
    Flavor::Chocolate => println!("Flavor: Chocolate"),
    Flavor::Cola => println!("Flavor: Cola"),
    Flavor::Strawberry => println!("Flavor: Strawberry"),
    Flavor::Vanilla => println!("Flavor: Vanilla"),
  }
  println!("oz: {:?}",drink.fluid_oz);
}

fn main() {
  let my_drink = Drink {
    flavor: Flavor::Chocolate,
    fluid_oz: 1.0,
  };
  print_drink(my_drink);
  let your_drink = Drink {
    flavor: Flavor::Cola,
    fluid_oz: 2.0,
  };
  print_drink(your_drink);
}
