// fn main() {
//   rary::public_function();
//   rary::indirect_access();
// }

// Compile as a binary crate using library.rlib:
// rustc runner.rs --extern rary=library.rlib

pub fn run() {
  rary_demo::public_function();
  rary_demo::indirect_access();
}