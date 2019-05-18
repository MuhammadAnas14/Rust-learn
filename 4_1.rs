// fn main() {
//     let s ="hello";
//     {
//       let s = "hi";
//     }
//     println!("{}",s);
// }


// fn main() {
//   let mut s = String::from("hello");
//   s.push_str(",world!");

//   println!("{}",s)
// }


// fn main() {
// {
//     let s = String::from("hello"); // s is valid from this point forward

//     // do stuff with s
// }                                  // this scope is now over, and s is no
//                                    // longer valid
// }



// fn main() {
//   let x = 5;
//   let y = x;
//   println!("{}",y);
// }

// this two example says that when s2 is defined to s1 
// s1 is out of the scope

// fn main() {
// let s1 = String::from("hello");
// let s2 = s1;
// }


// fn main(){
// let s1 = String::from("hello");
// let s2 = s1;

// println!("{}, world!", s1);
// }

// the better way to use the both s1 and s2
// fn main() {
// let s1 = String::from("hello");
// let s2 = s1.clone();

// println!("s1 = {}, s2 = {}", s1, s2);
// }

//in this example we see a integer vlue 
// fn main() {
// let x = 5;
// let y = x;

// println!("x = {}, y = {}", x, y);
// }

// fn main() {
//     let s = String::from("hello");  // s comes into scope

//     takes_ownership(s);             // s's value moves into the function...
//                                     // ... and so is no longer valid here

//     let x = 5;                      // x comes into scope

//     makes_copy(x);                  // x would move into the function,
//                                     // but i32 is Copy, so it’s okay to still
//                                     // use x afterward

// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.

// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{}' is {}.", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String

//     (s, length)
// }

