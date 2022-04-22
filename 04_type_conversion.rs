use std::convert::{From, TryFrom, TryInto};
use std::fmt;

#[derive(Debug)]
struct Number {
	value: i32,
}
// The Into trait is simply the reciprocal of the From trait. 
// That is, if you have implemented the From trait for your type, Into will call it when necessary.
impl From<i32> for Number {
	fn from(v: i32) -> Number {
		Number { value: v }
	}
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
	type Error = ();
	
	fn try_from(v: i32) -> Result<Self, Self::Error> {
		if v % 2 == 0 {
			Ok(EvenNumber(v))
		} else {
			Err(())
		}
	}
}

struct Circle {
	radius: i32,
}

impl From<i32> for Circle {
	fn from(v: i32) -> Self {
		Circle { radius: v }
	}
}

impl fmt::Display for Circle {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Circle of radius {}", self.radius)
	}
}

fn from_conversion() {
	println!("from_conversion");
	let int = 5;
	let num = Number::from(3);
	println!("My number is {:?}", num);
	
	let num: Number = int.into();
	println!("My number is {:?}", num);
}

fn try_from_conversion() {
	println!("try_from_conversion");
	assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
	assert_eq!(EvenNumber::try_from(7), Err(()));
	
	let result: Result<EvenNumber, ()> = 8i32.try_into();
	assert_eq!(result, Ok(EvenNumber(8)));
	let result: Result<EvenNumber, ()> = 7i32.try_into();
	assert_eq!(result, Err(()));
}

fn to_and_from_string() {
	println!("to_and_from_string");
	let circle = Circle::from(10);
	println!("{}", circle.to_string());
	
	let parsed: i32 = "5".parse().unwrap();
	let turbo_parsed = "10".parse::<i32>().unwrap();
	
	let sum = parsed + turbo_parsed;
	println!("Sum: {}", sum);
}

fn main() {
	from_conversion();
	try_from_conversion();
	to_and_from_string();
}