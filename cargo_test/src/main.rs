#[macro_use]
extern crate serde_json;
mod foo;
use foo::Foo;

macro_rules! for_each {
  (from $x:tt to $y:tt do $res:expr) => {{
    ($x..=$y).for_each($res);
  }};
}

macro_rules! matrix {       
  ([$($val:expr),*]) => (
      vec![$($val),*]
    );
    ([$($($val:expr),*);*]) => (
      vec![$(matrix!([$($val),*])),*]
    )


}

fn main() {
  println!("Hello, this program tests serde_json!");
  let john = json!({
      "name": "John Doe",
      "age": 43,
      "phones": [
        "+44 1234567",
        "+44 2345678"
      ],
      "test": {"hello":"testHello"}
    });

  println!("{:?}", john);

  let mut x = Foo::A("hi i'm inside A");
  println!("x: {}", translate(x));
  x = Foo::B;
  println!("x: {}", translate(x));
  x = Foo::C;
  println!("x: {}", translate(x));
  x = Foo::B;
  println!("x is an A: {}", is_pred(x, is_a));

  let x = 5;
  let fact = |n| (1..=n).fold(1, |acc, x| x * acc);
  println!("{:?}", fact(x));

  for_each!(from 3 to 5 do |x| println!("{:?}", x + 5));
  println!("{:?}", matrix!([1,2,3;4,5,6;7,8,9]));
}

fn translate(x: Foo) -> &'static str {
  match x {
    Foo::A(val) => val,
    Foo::B => "I'm a B",
    Foo::C => "I'm a C",
  }
}

fn is_a(x: Foo) -> bool {
  match x {
    Foo::A(val) => true,
    Foo::B => false,
    Foo::C => false,
  }
}

fn is_pred(x: Foo, f: fn(Foo) -> bool) -> bool {
  f(x)
}
