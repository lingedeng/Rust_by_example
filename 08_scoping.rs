	fn print_knowledage_point(s: &str) {
	println!("~~~~~~~{}~~~~~~~", s);
}

fn create_box() {
	let _box1 = Box::new(3i32);
	
	// `_box1` is destroyed here, and memory gets freed
}

struct ToDrop;

impl Drop for ToDrop {
	fn drop(&mut self) {
		println!("ToDrop is being dropped");
	}
}

fn raii() {
	println!("~Resource Acquisition Is Initialization~");
	let _box2 = Box::new(5i32);
	
	{
		let _box3 = Box::new(4i32);
	}
	
	for _ in 0u32..1_000 {
		create_box();
	}
	
	let x = ToDrop;
	println!("Make a ToDrop!");
}

fn partial_move() {
	print_knowledage_point("partial move");
	#[derive(Debug)]
	struct Person {
		name: String,
		age: u8,
	}
	
	let person = Person {
		name: String::from("Alice"),
		age: 20,
	};
	
	// struct destructate
	let Person { name, ref age } = person;
	// error[E0308]: mismatched types
	//let Person { name, age: &u8 } = person;
	// error: expected identifier, found `&`
	//let Person { name, &age } = person;
	println!("The person's age is {}", age);
	println!("The person's name is {}", name);
	
	// error[E0382]: borrow of partially moved value: `person`
	//println!("The person struct is {:?}", person);
	
	println!("The person's age from Person struct is {}", person.age);
}

fn eat_box_i32(boxed_i32: Box<i32>) {
	println!("Destroying box that contains {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
	println!("This int is: {}", borrowed_i32);
}

fn borrow() {
	print_knowledage_point("borrow");
	
	let boxed_i32 = Box::new(5i32);
	let stacked_i32 = 6i32;
	
	borrow_i32(&boxed_i32);
	borrow_i32(&stacked_i32);
	
	{
		let _ref_to_i32: &i32 = &boxed_i32;
		
		// error[E0505]: cannot move out of `boxed_i32` because it is borrowed
		//eat_box_i32(boxed_i32);
		
		borrow_i32(_ref_to_i32);
	}
	
	eat_box_i32(boxed_i32);
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
/*
error[E0277]: the trait bound `Book: Clone` is not satisfied
pub trait Copy: Clone {
                ^^^^^ required by this bound in `Copy`
*/
//#[derive(Copy)]
// error[E0382]: borrow of moved value: `immutabook`
//#[derive(Clone)]
struct Book {
	author: &'static str,
	title: &'static str,
	year: u32,
}

fn borrow_book(book: &Book) {
	println!("I immutably borrowed {} - {} edition", book.title, book.year);
}


fn new_edition(book: &mut Book) {
	book.year = 2021;
	println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn borrow_mutable() {
	print_knowledage_point("borrow mutable");
	
	let immutabook = Book {
        // string literals have type `&'static str`
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };
	
	let mut mutabook = immutabook;
	
	borrow_book(&immutabook);
	borrow_book(&mutabook);
	
	new_edition(&mut mutabook);
	// error[E0596]: cannot borrow `immutabook` as mutable, as it is not declared as mutable
	//new_edition(&mut immutabook);
}

// When doing pattern matching or destructuring via the let binding, 
// the ref keyword can be used to take references to the fields of a struct/tuple. 
#[derive(Clone, Copy)]
struct Point { x: i32, y: i32}

fn borrow_ref_pattern() {
	print_knowledage_point("borrow ref pattern");
	
	let c = 'Q';
	let d = 'Q';
	
	let ref ref_c1 = c;
	let ref_c2 = &d;
	println!("deref ref_c1 equals deref ref_c2: {}", *ref_c1 == *ref_c2);
	// FIXME: auto deref?
	println!("ref_c1 equals ref_c2: {}", ref_c1 == ref_c2);
	
	let point = Point { x: 3, y: 4};
	let copy_of_x = {
		// error[E0026]: struct `Point` does not have fields named `p1`, `p2`
		//let Point { ref p1, p2 } = point;
		// error: expected identifier, found reserved identifier `_`
		//let Point { ref x, _ } = point;
		
		// Ok, variable x,y same as Point's field name x, y
		//let Point { ref x, y } = point;
		// Ok too, warning: unused variable: `p2`
		//let Point { x: ref p1, y: p2 } = point;
		// Ok too, without prev warning
		let Point { x: ref p1, y: _ } = point;
		
		*p1
	};
	println!("copy_of_x is {}", copy_of_x);
	
	let mut mutable_point = point;
	{
		let Point { x: _, y: ref mut p2 } = mutable_point;
		
		*p2 = 5;
	}
	println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);
	
	let mut mutable_tuple = (Box::new(5u32), 3u32);
	println!("tuple is {:?}", mutable_tuple);
	{
		// Destructure `mutable_tuple` to change the value of `last`.
		let (_, ref mut t2) = mutable_tuple;
		*t2 = 2u32;
	}
	println!("tuple is {:?}", mutable_tuple);
	
}


//the borrow is valid as long as it ends before the lender is destroyed. (借用的生命周期[lifetime]为借用者被销毁之前)
//However, the scope of the borrow is determined by where the reference is used. (借用的范围[scope]由该借用被使用的地方确定)

// The lifetime is never constrained, it defaults to `'static`.
// 没有参数的函数使用生命周期标识表示该函数的生命周期（通常为'static）,而_x并没有'static如此长的生命周期
fn fail_borrow<'a>() {	
	let _x = 32;
	
	// error[E0597]: `_x` does not live long enough
	//let y: &'a i32 = &_x;
}

fn ok_borrow<'a>() {
	println!("~~~~~~~Ok borrow~~~~~~~");
	static x: i32 = 32; // const is ok too
	
	let y: &'a i32 = &x;
	
	println!("y: {}", y);
}

// `T: 'a`: *All* references in `T` must outlive lifetime `'a`.
// `T: Trait + 'a`: Type `T` must implement trait `Trait` and *all* references in `T` must outlive `'a`.
use std::fmt::Debug;

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// `Ref` contains a reference to a generic type `T` that has
// an unknown lifetime `'a`. `T` is bounded such that any
// *references* in `T` must outlive `'a`. Additionally, the lifetime
// of `Ref` may not exceed `'a`.

fn print<T>(t: T) where
	T: Debug {

	println!("'print': t is {:?}", t);
}

// Here a reference to `T` is taken where `T` implements
// `Debug` and all *references* in `T` outlive `'a`. In
// addition, `'a` must outlive the function.
fn print_ref<'a, T>(t: &'a T)
	where T: Debug + 'a {

	println!("'print_ref': t is {:?}", t);
}

fn lifetime_bounds() {
	println!("~~~~~~~lifetime bounds~~~~~~~");
	
	let x = 7;
	let ref_x = Ref(&x);
	
	print_ref(&ref_x);
	print(ref_x);
}

//A longer lifetime can be coerced into a shorter one so that it works inside a scope it normally wouldn't work in.

// Here, Rust infers a lifetime that is as short as possible.
// The two references are then coerced to that lifetime.
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
	first * second
}

// `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`.
// Here, we take in an `&'a i32` and return a `&'b i32` as a result of coercion.
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
	first
}

fn lifetime_coercion() {
	println!("~~~~~~~lifetime coercion~~~~~~~");
	let first = 2;
	{
		let second = 4;
		
		println!("The product is {}", multiply(&first, &second));
		println!("{} is the first", choose_first(&first, &second));
	}
}

// As a reference lifetime 'static indicates that the data pointed to by the reference lives for the entire lifetime of the running program. 
// It can still be coerced to a shorter lifetime.

// As a trait bound, it means the type does not contain any non-static references. 
// Eg. the receiver can hold on to the type for as long as they want and it will never become invalid until they drop it.
// It's important to understand this means that any owned data always passes a 'static lifetime bound, 
// but a reference to that owned data generally does not.
fn print_it(input: impl Debug + 'static) {
	println!("'static value passed in is: {:?}", input);
}

fn lifetime_trait_static_bounds() {
	println!("~~~~~~~lifetime trait static bounds~~~~~~~");
	// i is owned and contains no references, thus it's 'static:
	let x = 1;
	print_it(x);
	
	// oops, &i only has the lifetime defined by the scope of
    // main(), so it's not 'static:
	//print_it(&x);
}

fn main() {
	raii();
	
	partial_move();
	
	borrow();
	borrow_mutable();
	borrow_ref_pattern();
	
	ok_borrow();
	lifetime_bounds();
	lifetime_coercion();
	lifetime_trait_static_bounds();
}