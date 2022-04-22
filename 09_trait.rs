fn print_knowledage_point(s: &str) {
	println!("~~~~~~~{}~~~~~~~", s);
}

struct Sheep { naked: bool, name: &'static str }

trait Animal {
	fn new(name: &'static str) -> Self;
	
	fn name(&self) -> &'static str;
	fn noise(&self) -> &'static str;
	
	fn talk(&self) {
		println!("{} says {}", self.name(), self.noise());
	}
}

impl Sheep {
	fn is_naked(&self) -> bool {
		self.naked
	}
	
	fn shear(&mut self) {
		if self.is_naked() {
			println!("{} is already naked...", self.name());
		} else {
			println!("{} gets a haircut", self.name());
			
			self.naked = true;
		}
	}
}

impl Animal for Sheep {
	fn new(name: &'static str) -> Sheep {
		Sheep { name, naked: false }
	}
	
	fn name(&self) -> &'static str {
		self.name
	}
	
	fn noise(&self) -> &'static str {
		if self.is_naked() {
			"baaaaah?"
		} else {
			"baaaaah!"
		}
	}
	
	fn talk(&self) {
		println!("{} pauses briefly... {}", self.name, self.noise());
	}
}

fn trait_example() {
	print_knowledage_point("trait example");
	
	// error[E0282]: type annotations needed
	//let mut dolly/*: Sheep*/ = Animal::new("Dolly");    
	let mut dolly: Sheep = Animal::new("Dolly");
	// Ok
	//let mut dolly = Sheep::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();
}

/*
The compiler is capable of providing basic implementations for some traits via the #[derive] attribute.

The following is a list of derivable traits:

Comparison traits: Eq, PartialEq, Ord, PartialOrd.
Clone, to create T from &T via a copy.
Copy, to give a type 'copy semantics' instead of 'move semantics'.
Hash, to compute a hash from &T.
Default, to create an empty instance of a data type.
Debug, to format a value using the {:?} formatter.
*/
#[derive(PartialEq, PartialOrd, Debug)]
struct Centimeter(f64);

fn trait_derive() {
	print_knowledage_point("trait derive");
	
	let cm1 = Centimeter(100.0);
	let cm2 = Centimeter(101.0);
	
	let cmp = if cm1 < cm2 {
		"smaller"
	} else {
		"bigger"
	};
	println!("cm1: {:?} is {} than cm2: {:?}", cm1, cmp, cm2);
}

/*
The Rust compiler needs to know how much space every function's return type requires. 
This means all your functions have to return a concrete type.
Instead of returning a trait object directly, our functions return a Box which contains some Animal.
So if your function returns a pointer-to-trait-on-heap in this way, you need to write the return type with the dyn keyword, e.g. Box<dyn Animal>.
*/
struct Cat {}
struct Cow {}

trait Animal2 {
	fn noise(&self) -> &'static str;
}

//impl Animal for Cat {
impl Animal2 for Cat {
	fn noise(&self) -> &'static str {
		"meooooow"
	}
}

//impl Animal for Cow {
impl Animal2 for Cow {	
	fn noise(&self) -> &'static str {
		"mooooooo"
	}
}

// error[E0038]: the trait `Animal` cannot be made into an object
// ^^^ ...because associated function `new` has no `self` parameter
//fn random_animal(v: f64) -> Box<dyn Animal> {
fn random_animal(v: f64) -> Box<dyn Animal2> {
	if v < 0.5 {
		Box::new(Cat {})
	} else {
		Box::new(Cow {})
	}
}

fn trait_as_return() {
	print_knowledage_point("trait as return");
	
	let v = 0.234;
	let animal = random_animal(v);
	println!("You've randomly chosen an animal, and it says {}", animal.noise());
}

/*
In Rust, many of the operators can be overloaded via traits. 
That is, some operators can be used to accomplish different tasks based on their input arguments.
This is possible because operators are syntactic sugar for method calls. 
For example, the + operator in a + b calls the add method (as in a.add(b))
A list of the traits, such as Add, that overload operators can be found in std::ops

The Drop trait only has one method: drop, which is called automatically when an object goes out of scope. 
The main use of the Drop trait is to free the resources that the implementor instance owns.
Box, Vec, String, File, and Process are some examples of types that implement the Drop trait to free resources.
*/
use std::ops;
struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;
#[derive(Debug)]
struct BarFoo;

// The `std::ops::Add` trait is used to specify the functionality of `+`.
// Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
// The following block implements the operation: Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
	type Output = FooBar;
	
	fn add(self, _rhs: Bar) -> Self::Output {
		println!("> Foo.add(Bar) was called");
		FooBar
	}
}

impl ops::Add<Foo> for Bar {
	type Output = BarFoo;
	
	fn add(self, _rhs: Foo) -> Self::Output {
		println!("> Bar.add(Foo) was called");
		BarFoo
	}
}

struct Droppable {
	name: &'static str,
}

impl Drop for Droppable {
	fn drop(&mut self) {
		println!("> dropping {}", self.name);
	}
}

fn trait_op_overload() {
	print_knowledage_point("trait op overload & drop");
	
	println!("Foo + Bar = {:?}", Foo + Bar);
	println!("Bar + Foo = {:?}", Bar + Foo);
	
	println!("===drop op overload===");
	let _a = Droppable { name: "a" };

    // block A
    {
        let _b = Droppable { name: "b" };

        // block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");
	
	// Variable can be manually dropped using the `drop` function
	std::mem::drop(_a);
	
	println!("end of the main function");
}

/*
The Iterator trait is used to implement iterators over collections such as arrays.
The trait requires only a method to be defined for the next element, which may be manually defined in an impl block or automatically defined (as in arrays and ranges).
As a point of convenience for common situations, the for construct turns some collections into iterators using the .into_iter() method.
*/
struct Fibonacci {
	curr: u32,
	next: u32,
}

// Here, we define the sequence using `.curr` and `.next`.
// The return type is `Option<T>`:
//     * When the `Iterator` is finished, `None` is returned.
//     * Otherwise, the next value is wrapped in `Some` and returned.
// We use Self::Item in the return type, so we can change
// the type without having to update the function signatures.
impl Iterator for Fibonacci {
	type Item = u32;
	
	fn next(&mut self) -> Option<Self::Item> {
		let new_next = self.curr + self.next;
		
		self.curr = self.next;
		self.next = new_next;
		
		Some(self.curr)
	}
}

fn fibonacci() -> Fibonacci {
	Fibonacci{ curr: 0, next: 1 }
}

fn trait_iterator() {
	print_knowledage_point("trait iterator");
	
	let mut sequence = 0..3;
	println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
	
	// `for` works through an `Iterator` until it returns `None`.
    // Each `Some` value is unwrapped and bound to a variable (here, `i`).
    println!("Iterate through 0..3 using `for`");
	for i in 0..3 {
		println!("> {}", i);
	}
	
	// The `take(n)` method reduces an `Iterator` to its first `n` terms.
    println!("The first four terms of the Fibonacci sequence are: ");
	for i in fibonacci().take(4) {
		println!("> {:?}", i); 
	}
	
	// The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
    println!("The next four terms of the Fibonacci sequence are: ");
	for i in fibonacci().skip(4).take(4) {
		println!("> {}", i);
	}
		
	let array = [1u32, 3, 3, 7];
	println!("Iterate the following array {:?}", &array);
	// The `iter` method produces an `Iterator` over an array/slice.
	for i in array.iter() {
		println!("> {}", i);
	}
}

/*
impl Trait can be used in two locations:

1. as an argument type
2. as a return type
*/
// TODO: impl trait example
#[allow(dead_code)]
//fn parse_csv_document<R: std::io::BufRead>(_src: R) -> std::io::Result<Vec<Vec<String>>> {
fn parse_csv_document(_src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
	Err(std::io::Error::last_os_error())
}

use std::iter;
use std::vec::IntoIter;

#[allow(dead_code)]
fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
/*
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}
*/
) -> impl Iterator<Item=i32> {
	v.into_iter().chain(u.into_iter()).cycle()
}

#[derive(Debug, Clone, Copy)]
struct Unit;

#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn trait_clone() {
	print_knowledage_point("trait clone");
	
	let unit = Unit;
	let copied_unit = unit;
	
	println!("original: {:?}", unit);
    println!("copy: {:?}", copied_unit);
	
	let pair = Pair(Box::new(1), Box::new(2));
	println!("Pair original: {:?}", pair);
	
	// Move `pair` into `moved_pair`, moves resources
	let moved_pair = pair;
	println!("moved pair: {:?}", moved_pair);
	
	// error[E0382]: borrow of moved value: `pair`
	//println!("Pair original: {:?}", pair);
	
	let clone_pair = moved_pair.clone();
	std::mem::drop(moved_pair);
	// error[E0382]: borrow of moved value: `moved_pair`
	// Error! `moved_pair` has been dropped
	//println!("moved pair: {:?}", moved_pair);
	
	println!("cloned pair: {:?}", clone_pair);	
}

trait Person {
	fn name(&self) -> String;
}

// Person is a supertrait of Student.
// Implementing Student requires you to also impl Person.
trait Student: Person {
	fn university(&self) -> String;
}

trait Programmer {
	fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
	fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
	format!(
		"My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
		student.name(),
		student.university(),
		student.fav_language(),
		student.git_username())
}

struct Whoami;

impl CompSciStudent for Whoami {
	fn git_username(&self) -> String {
		String::from("whoami@gmail.com")
	}
}

impl Programmer for Whoami {
	fn fav_language(&self) -> String {
		String::from("Rust")
	}
}

impl Student for Whoami {	
	fn university(&self) -> String {
		String::from("UCLA")
	}
}

impl Person for Whoami {
	fn name(&self) -> String {
		String::from("Bob")
	}
}

fn trait_superset() {
	print_knowledage_point("trait superset");
	
	let whoami = Whoami;
	println!("{}", comp_sci_student_greeting(&whoami));
}

trait NameWidget {
	fn get(&self) -> String;
}

trait AgeWidget {
	fn get(&self) -> u8;
}

struct Form {
	name: String,
	age: u8,
}

impl NameWidget for Form {
	fn get(&self) -> String {
		self.name.clone()
	}	
}

impl AgeWidget for Form {
	fn get(&self) -> u8 {
		self.age
	}
}

fn trait_fn_with_same_name() {
	print_knowledage_point("trait fn with same name");
	
	let form = Form {
		name: String::from("rustacean"),
		age: 28,
	};
		
	// error[E0034]: multiple applicable items in scope
	//println!("{}", form.get());
	
	// Fully Qualified Syntax
	let name = <Form as NameWidget>::get(&form);	
	let age = <Form as AgeWidget>::get(&form);
	
	println!("name is {}, age is {}", name, age);
}

fn main() {
	trait_example();
	trait_derive();
	trait_as_return();
	trait_op_overload();
	trait_iterator();
	trait_clone();
	trait_superset();
	trait_fn_with_same_name();
}