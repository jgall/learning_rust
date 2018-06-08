use std::collections::BTreeMap;

use std::sync::{Arc, Mutex};
use std::thread::spawn;

macro_rules! vector {
  (from $left:tt to $right:tt) => {{
    let mut temp_vec = Vec::new();
    for x in ($left)..($right) {
      temp_vec.push(x);
    }
    temp_vec
  }};
  (from $left:tt through $right:tt) => {{
    let mut temp_vec = Vec::new();
    for x in ($left)..=($right) {
      temp_vec.push(x);
    }
    temp_vec
  }};
}

macro_rules! matrix { 
   ([ $( $val:expr ),* ]) => {
    vec![$($val),*]
  };
  ([ $( $( $val:expr ),*);* ]) => {
    vec![$(matrix!([$($val),*])),*]
  };
}

macro_rules! foreach {
  ($it:ident in $from:tt to $to:tt do $e:expr) => {{
    for $it in $from..$to {
      $e;
    }
  }};
}

macro_rules! style {
  ($($attr:tt: $value:expr;)*) => {{
    let mut style = Style::new();
    $( style.insert(stringify!($attr).to_owned(), $value.to_style_value()); )*
    style
  }}
}

#[derive(Debug)]
enum StyleValue {
  String(String),
  Int32(i32),
}

trait ToStyleValue {
  fn to_style_value(self) -> StyleValue;
}

impl ToStyleValue for i32 {
  fn to_style_value(self) -> StyleValue {
    StyleValue::Int32(self)
  }
}

impl ToStyleValue for String {
  fn to_style_value(self) -> StyleValue {
    StyleValue::String(self)
  }
}

impl ToStyleValue for &'static str {
  fn to_style_value(self) -> StyleValue {
    StyleValue::String(self.to_owned())
  }
}

#[derive(Debug)]
struct Style(BTreeMap<String, StyleValue>);

impl Style {
  fn new() -> Self {
    Style(BTreeMap::new())
  }

  fn insert(&mut self, attr: String, val: StyleValue) {
    self.0.insert(attr, val);
  }
}

fn main() {
  let fact = |x| (1..=x).fold(1, |acc, n| acc * n);
  println!("{:?}", fact(5));

  let free = |_| {};
  let y = "hello".to_owned();
  free(&y);
  println!("{}", y);

  let y_ref1 = Arc::new(Mutex::new(y));
  let y_ref2 = y_ref1.clone();

  let t1 = spawn(move || {
    (1..=100).for_each(|x| println!("{:?}, y: {}", x,));
  });
  let t2 = spawn(move || {
    (1..=100).for_each(|x| println!("{:?}, y: {}", x, y_ref2.as_ref().lock().unwrap()));
  });
  let _ = (t1.join(), t2.join());

  foreach!(x in 1 to 5 do println!("hello {:?}", x));

  println!("{:?}", vector!(from 1 to 4));
  println!("{:?}", matrix!([1, 2, 3; 4, 5, 6]));
  println!("{:?}", matrix!([1, 2, 3, 4, 5, 6]));
  println!(
    "{:?}",
    style!{
      width: 10+5;
      height: "50";
    }
  );
}
