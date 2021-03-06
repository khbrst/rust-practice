mod hashmap;
mod result;
mod strings;

pub fn box_and_stack_and_heap() {
  use std::mem;

  #[allow(dead_code)]
  #[derive(Debug, Clone, Copy)]
  struct Point {
    x: f64,
    y: f64,
  }

  #[allow(dead_code)]
  struct Rectangle {
    p1: Point,
    p2: Point,
  }

  fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
  }

  fn boxed_origin() -> Box<Point> {
    // Allocate this point in the heap, and return a pointer to it
    Box::new(Point { x: 0.0, y: 0.0 })
  }

  // (all the type annotations are superfluous)
  // Stack allocated variables
  let point: Point = origin();
  let rectangle: Rectangle = Rectangle {
    p1: origin(),
    p2: Point { x: 3.0, y: 4.0 },
  };

  // Heap allocated rectangle
  let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
    p1: origin(),
    p2: origin(),
  });

  // The output of functions can be boxed
  let boxed_point: Box<Point> = Box::new(origin());

  // Double indirection
  let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

  println!(
    "Point occupies {} bytes in the stack",
    mem::size_of_val(&point)
  );
  println!(
    "Rectangle occupies {} bytes in the stack",
    mem::size_of_val(&rectangle)
  );

  // box size = pointer size
  println!(
    "Boxed point occupies {} bytes in the stack",
    mem::size_of_val(&boxed_point)
  );
  println!(
    "Boxed rectangle occupies {} bytes in the stack",
    mem::size_of_val(&boxed_rectangle)
  );
  println!(
    "Boxed box occupies {} bytes in the stack",
    mem::size_of_val(&box_in_a_box)
  );

  // Copy the data contained in `boxed_point` into `unboxed_point`
  let unboxed_point: Point = *boxed_point;
  println!(
    "Unboxed point occupies {} bytes in the stack",
    mem::size_of_val(&unboxed_point)
  );
}

pub fn vectors() {
  // Iterators can be collected into vectors
  let collected_iterator: Vec<i32> = (0..10).collect();
  println!("Collected (0..10) into: {:?}", collected_iterator);

  // The `vec!` macro can be used to initialize a vector
  let mut xs = vec![1i32, 2, 3];
  println!("Initial vector: {:?}", xs);

  // Insert new element at the end of the vector
  println!("Push 4 into the vector");
  xs.push(4);
  println!("Vector: {:?}", xs);

  // Error! Immutable vectors can't grow
  // collected_iterator.push(0);
  // FIXME ^ Comment out this line

  // The `len` method yields the current size of the vector
  println!("Vector size: {}", xs.len());

  // Indexing is done using the square brackets (indexing starts at 0)
  println!("Second element: {}", xs[1]);

  // `pop` removes the last element from the vector and returns it
  println!("Pop last element: {:?}", xs.pop());

  // Out of bounds indexing yields a panic
  // println!("Fourth element: {}", xs[3]);
  // FIXME ^ Comment out this line

  // `Vector`s can be easily iterated over
  println!("Contents of xs:");
  for x in xs.iter() {
    println!("> {}", x);
  }

  // A `Vector` can also be iterated over while the iteration
  // count is enumerated in a separate variable (`i`)
  for (i, x) in xs.iter().enumerate() {
    println!("In position {} we have value {}", i, x);
  }

  // Thanks to `iter_mut`, mutable `Vector`s can also be iterated
  // over in a way that allows modifying each value
  for x in xs.iter_mut() {
    *x *= 3;
  }
  println!("Updated vector: {:?}", xs);
}

pub fn run_strings() {
  strings::basic();
  strings::string_literals_and_escapes();
  strings::raw_string_literals();
  strings::byte_strings();
}

pub fn option() {
  // An integer division that doesn't `panic!`
  fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
      // Failure is represented as the `None` variant
      None
    } else {
      // Result is wrapped in a `Some` variant
      Some(dividend / divisor)
    }
  }

  // This function handles a division that may not succeed
  fn try_division(dividend: i32, divisor: i32) {
    // `Option` values can be pattern matched, just like other enums
    match checked_division(dividend, divisor) {
      None => println!("{} / {} failed!", dividend, divisor),
      Some(quotient) => println!("{} / {} = {}", dividend, divisor, quotient),
    }
  }

  try_division(4, 2);
  try_division(1, 0);

  // Binding `None` to a variable needs to be type annotated
  let _none: Option<i32> = None;
  let _equivalent_none = None::<i32>;

  let optional_float = Some(0f32);

  // Unwrapping a `Some` variant will extract the value wrapped.
  println!(
    "{:?} unwraps to {:?}",
    optional_float,
    optional_float.unwrap()
  );

  // Unwrapping a `None` variant will `panic!`
  // println!("{:?} unwraps to {:?}", _none, _none.unwrap());
}

pub fn run_result() {
  result::basic();
  result::question_mark();
}

pub fn run_hashmap() {
  hashmap::basic();
  hashmap::alternate_custom_key_types();
  hashmap::hashset();
}
