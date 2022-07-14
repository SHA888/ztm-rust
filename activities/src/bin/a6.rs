// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:
// * Use a mutable integer variable
// * Use a while statement
// * Print the variable within the while loop
// * Do not use break to exit the loop

fn main() {
  let mut my_int = 5;

  while my_int >= 1 {
    println!("{}", my_int);
    my_int = my_int - 1;
  }
  
  println!("done!");
}
