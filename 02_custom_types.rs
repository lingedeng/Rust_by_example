#![allow(dead_code)]
#![allow(unused_imports)]

#[derive(Debug)]
struct Point3D {
	x: i64,
	y: i64,
	z: i64,
}

#[derive(Debug)]
enum WebEvent {
	PageLoad,
	PageUnload,
	KeyPress(char),
	Paste(String),
	Click { x: i64, y: i64},
	Draw(Point3D),
}

enum Status {
	Rich,
	Poor,
}

enum Work {
	Civilian,
	Soldier,
}

// enum with implicit discriminator (starts at 0)
#[derive(Debug)]
enum Number {
	Zero,
	One,
	Two,
}

enum Color {
	Red = 0xff0000,
	Green = 0x00ff00,
	Blue = 0x0000ff,
}

impl WebEvent {
	fn inspect(&self) {
		//match *self {
		use crate::WebEvent::*;
		match *self {
			WebEvent::PageLoad => println!("Page load"),
			WebEvent::PageUnload => println!("Page unload"),
			WebEvent::KeyPress(key) => println!("pressed '{}'", key),
			WebEvent::Paste(ref s) => println!("pasted \"{}\"", s),
			WebEvent::Click { x, y } => {
				println!("clicked at x={}, y={}", x, y);
			},
			WebEvent::Draw(ref p) => {
				println!("draw at x={}, y={}, z={}", p.x, p.y, p.z);
			},
			//_ => println!("Unknown event!"),
		}
	}
}

enum List {
	Cons(u32, Box<List>),
	Nil,
}

use crate::List::*;

impl List {
	fn new() -> List {
		Nil
	}
	
	fn prepend(self, elem: u32) -> List {
		Cons(elem, Box::new(self))
	}
	
	fn len(&self) -> u32 {
		// `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        // after Rust 2018 you can use self here and tail (with no ref) below as well,
        // rust will infer &s and ref tail. 
        // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
		match *self {
			// Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
			Cons(_, ref tail) => 1 + tail.len(),
			Nil => 0,
		}
	}
	
	fn stringify(&self) -> String {
		match self {
			Cons(v, tail) => {
				format!("{}, {}", v, tail.stringify())
			},
			Nil => {
				format!("Nil")
			},
		}
	}
}

static LANG: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
	n > THRESHOLD
}

fn main() {
	let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    //let pasted  = WebEvent::Paste("my text".to_owned());
	let pasted = WebEvent::Paste(String::from("my text"));
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;
	let draw = WebEvent::Draw(Point3D { x: 10, y: 20, z:-5 });
	
	pressed.inspect();
	pasted.inspect();
	click.inspect();
	load.inspect();
	unload.inspect();
	draw.inspect();
	
	println!("{:?}", pressed);
	println!("{:?}", pasted);
	println!("{:?}", click);
	println!("{:?}", load);
	println!("{:?}", unload);
	println!("{:?}", draw);
	
	println!("~~~~~~~use~~~~~~~");
	use crate::Status::{Rich, Poor};
	use crate::Work::*;
	
	let status = Poor;
	let work = Soldier;
	
	match status {
		Rich => println!("The rich have lots of money!"),
		Poor => println!("The poor have no money..."),
	}
	
	match work {
		Civilian => println!("hard work"),
		Soldier => println!("fight"),
	}
	
	println!("~~~~~~~C-like enum~~~~~~~");
	println!("Zero is {:?}", Number::Zero);
	println!("Two is {}", Number::Two as i32);
	println!("roses are #{:06x}", Color::Red as i32);
	println!("violets are #{:06x}", Color::Blue as i32);
	
	println!("~~~~~~~enum list~~~~~~~");
	// Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
	
	println!("~~~~~~~static & const~~~~~~~");
	let n = 17;
	println!("This is {}", LANG);
	println!("The threshold is {}", THRESHOLD);
	println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
	
	//THRESHOLD = 5;
}