// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
  name: String,
  locker: Option<i32>,
}

fn main() {
  let student_1 = Student {
    name: "Alice".to_owned(),
    locker: Some(1),
  };
  println!("Student 1 name: {:?}", student_1.name);
  match student_1.locker {
    Some(num) => println!("Student 1 locker number is {}", num),
    None => println!("Student 1 has no locker assigned"),
  }
}
