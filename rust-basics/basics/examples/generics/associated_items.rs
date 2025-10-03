struct Container(i32, i32);

trait Contains<A, B> {
  fn contains(&self, _: &A, _:&B) -> bool;
  fn first(&self) -> i32;
  fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
  fn contains(&self, number_1:&i32, number_2:&i32) -> bool {
    (&self.0 == number_1) && (&self.1 == number_2)
  }
  fn first(&self) -> i32 { self.0 }
  fn last(&self) -> i32 { self.1 }
}

// The Problem:
// `C` contains `A` and `B`. In light of that, having to express `A` and `B` again is a nuisance.
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> {
    container.last() - container.first()
}

// `A` and `B` are defined in the trait via the `type` keyword.
// (Note: `type` in this context is different from `type` when used for
// aliases).
trait ContainsAssociated {
  type A;
  type B;
  // Updated syntax to refer to these new types generically.
  fn contains_associated(&self, _: &Self::A, _: &Self::B) -> bool;
  fn first_associated(&self) -> i32;
  fn last_associated(&self) -> i32;
}

impl ContainsAssociated for Container {
  type A = i32;
  type B = i32;

  fn contains_associated(&self, number_1:&i32, number_2:&i32) -> bool {
    (&self.0 == number_1) && (&self.1 == number_2)
  }
  fn first_associated(&self) -> i32 { self.0 }
  fn last_associated(&self) -> i32 { self.1 }
}

fn difference_associated<C: ContainsAssociated>(container: &C) -> i32 {
    container.last_associated() - container.first_associated()
}

fn problem_example() {
  let number_1 = 3;
  let number_2 = 10;

  let container = Container(number_1, number_2);

  println!("Does container contain {} and {}: {}",
      &number_1, &number_2,
      container.contains(&number_1, &number_2));
  println!("First number: {}", container.first());
  println!("Last number: {}", container.last());

  println!("The difference is: {}", difference(&container));
}

fn associated_types_example() {
  let number_1 = 3;
  let number_2 = 10;

  let container = Container(number_1, number_2);

  println!("Does container contain {} and {}: {}",
      &number_1, &number_2,
      container.contains_associated(&number_1, &number_2));
  println!("First number: {}", container.first_associated());
  println!("Last number: {}", container.last_associated());
  
  println!("The difference is: {}", difference_associated(&container));
}

pub fn associated_items_demo() {
  // problem_example();
  associated_types_example();
}