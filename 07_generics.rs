struct SGen<T>(T);

fn generic<T: std::fmt::Display>(s: SGen<T>) {
	println!("s value is {}", s.0);
}

fn functions() {
	println!("~~~~~~~func~~~~~~~");
	let sgen_c: SGen::<char> = SGen('z'); // equals to SGen<char>
	generic::<char>(SGen('a'));
	generic(SGen::<u32>(12));
	generic(sgen_c);
	generic(SGen(6));
}

struct Val {
	val: f64,
}

struct GenVal<T> {
	gen_val: T,
}

impl Val {
	fn value(&self) -> &f64 {
		&self.val
	}
}

impl<T> GenVal<T> {
	// error: `default` is only allowed on items in trait impls
	//default fn value(&self) -> &T {
	fn value(&self) -> &T {
		&self.gen_val
	}
}

/*
error[E0592]: duplicate definitions with name `value`
impl GenVal<char> {
	fn value(&self) -> &char {
		println!("Use char as type parameter");
		&self.gen_val
		
	}
}
*/

fn impl_func() {
	println!("~~~~~~~impl func~~~~~~~");
	let x = Val { val: 3.0 };
	let y = GenVal { gen_val: 3i32 };
	let z = GenVal { gen_val: 'A' };
	
	println!("{}, {}, {}", x.value(), y.value(), z.value());
}

struct Empty;
struct Null;

trait DoubleDrop<T> {
	fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
	fn double_drop(self, _: T) {
		println!("double drop trait impl");
	}
}

trait GenValTrait<T> {
	fn value(&self) -> &T;
}

#[derive(Debug)]
struct MyGenVal<T> {
	gen_val: T,
}

impl<T> GenValTrait<T> for MyGenVal<T>	
{
	fn value(&self) -> &T {
		&self.gen_val
	}
}

/* FIXME
// error[E0119]: conflicting implementations of trait `GenValTrait<char>` for type `MyGenVal<char>`
impl GenValTrait<char> for MyGenVal<char> {
	fn value(&self) -> &char {
		println!("Use char as type parameter");
		//GenValTrait::<T>::value(self)
		&self.gen_val
	}
}
*/

fn impl_trait() {
	println!("~~~~~~~impl trait~~~~~~~");
	let empty = Empty;
	let null = Null;
	
	empty.double_drop(null);
	
	let a = MyGenVal { gen_val: 8i32 };
	let b = MyGenVal { gen_val: 'Z' };
	
	println!("{:?}, {:?}", a, b);
	
	//empty;
	//null;
}

/*
When working with generics, the type parameters often must use traits as bounds to stipulate(规定，明确要求) what functionality a type implements.
Bounding restricts the generic to types that conform to the bounds.
Another effect of bounding is that generic instances are allowed to access the methods of traits specified in the bounds.
T: trait1表示类型T需要声明trait1
*/
trait HasArea {
	fn area(&self) -> f64;
}

impl HasArea for Rectangle {
	fn area(&self) -> f64 { self.width * self.height }
}

#[derive(Debug)]
struct Rectangle { width: f64, height: f64, }
#[allow(dead_code)]
struct Triangle { length: f64, height: f64, }

fn print_debug<T: std::fmt::Debug>(t: &T) {
	println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 {
	t.area()
}

fn bound_trait() {
	println!("~~~~~~~bound trait~~~~~~~");
	let rectangle = Rectangle { width: 3.0, height: 4.0 };
	let _triangle = Triangle { length: 3.0, height: 4.0 };
	
	print_debug(&rectangle);
	println!("Area: {}", area(&rectangle));
	
	// error[E0277]: `Triangle` doesn't implement `Debug`
	//print_debug(&_triangle);
	// error[E0277]: the trait bound `Triangle: HasArea` is not satisfied
	//println!("Area: {}", area(&_triangle));
}

// A consequence of how bounds work is that even if a trait doesn't include any functionality, you can still use it as a bound. 
// Eq and Copy are examples of such traits from the std library.
struct Cardinal;
struct BlueJay;
struct Turkey;

trait Blue {}
trait Red {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

fn red<T: Red>(_: &T) -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

fn bound_empty_trait() {
	println!("~~~~~~~bound empty trait~~~~~~~");
	let cardinal = Cardinal;
	let bluejay = BlueJay;
	let _turkey = Turkey;
	
	println!("A cardinal is {}", red(&cardinal));
	println!("A bluejay is {}", blue(&bluejay));
	// error[E0277]: the trait bound `Turkey: Red` is not satisfied
	//println!("A turkey is {}", red(&_turkey));
}

use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
	println!("Debug: {:?}", t);
	println!("Display: {}", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
	println!("t: '{:?}'", t);
	println!("u: '{:?}'", u);
}

fn bound_multi_trait() {
	println!("~~~~~~~bound multi trait~~~~~~~");
	let string = "word";
	let array = [1, 2, 3];
	let vec = vec![1, 2, 3];
	
	compare_prints(&string);
	// error[E0277]: `[{integer}; 3]` doesn't implement `std::fmt::Display`
	//compare_prints(&array);
	// error[E0277]: `Vec<{integer}>` doesn't implement `std::fmt::Display`
	//compare_prints(&vec);
	
	compare_types(&array, &vec);
}

trait PrintInOption {
	fn print_in_option(self);
}

// error: expected `::`, found `PrintInOption`
//impl<Option<T>> PrintInOption for Option<T> {
// error: expected one of `!`, `+`, `::`, `>`, or `as`, found `:`
//impl<Option<T>: Debug> PrintInOption for Option<T> {

// TODO: understand this
// Because we would otherwise have to express this as `T: Debug` or 
// use another method of indirect approach, this requires a `where` clause:
impl<T> PrintInOption for T
	where Option<T>: Debug {
	
	// We want `Option<T>: Debug` as our bound because that is what's
    // being printed. Doing otherwise would be using the wrong bound.
	fn print_in_option(self) {
		println!("{:?}", Some(self));
	}
}

fn bound_where_clauses() {
	println!("~~~~~~~bound multi trait~~~~~~~");
	
	let vec = vec![1, 2, 3];
	vec.print_in_option();
}

struct Container(i32, i32);

trait Contains<A, B> {
	fn contains(&self, _: &A, _: &B) -> bool;
	fn first(&self) -> i32;
	fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
	fn contains(&self, num1: &i32, num2: &i32) -> bool {
		// error[E0308]: mismatched types, ^^^^ expected `i32`, found `&i32`
		//(self.0 == num1) && (&self.1 == num2)
		
		//(&self.0 == num1) && (&self.1 == num2)
		(self.0 == *num1) && (self.1 == *num2)
	}
	
	fn first(&self) -> i32 { self.0 }
	fn last(&self) -> i32 { self.1 }
}

fn difference<A, B, C>(container: &C) -> i32
	where C: Contains<A, B> {

	container.last() - container.first()
}

// FIXME: This example is not good enough
struct Container2(i32, i32);

trait Contains2 {
	type A;
	type B;
	
	fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
	fn first(&self) -> i32;
	fn last(&self) -> i32;
}

impl Contains2 for Container2 {
	type A = i32;
	type B = i32;
	
	fn contains(&self, num1: &i32, num2: &i32) -> bool {
		// error[E0308]: mismatched types, ^^^^ expected `i32`, found `&i32`
		//(self.0 == num1) && (&self.1 == num2)
		
		(&self.0 == num1) && (&self.1 == num2)		
	}
	
	fn first(&self) -> i32 { self.0 }
	fn last(&self) -> i32 { self.1 }
}

fn difference2<C: Contains2>(container: &C) -> i32 {
	container.last() - container.first()
}

fn without_associated_types() {
	println!("~~~~~~~without associated item~~~~~~~");
	let num1 = 3;
	let num2 = 10;
	
	let container = Container(num1, num2);
	println!("num1: {}, num2: {}", num1, num2);
	
	println!("Does container contain {} and {}: {}",
        &num1, &num2,
        container.contains(&num1, &num2));
		
	println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}

fn associated_types() {
	println!("~~~~~~~associated item~~~~~~~");
	let num1 = 3;
	let num2 = 10;
	
	let container = Container2(num1, num2);
	println!("num1: {}, num2: {}", num1, num2);
	
	println!("Does container2 contain {} and {}: {}",
        &num1, &num2,
        container.contains(&num1, &num2));
		
	println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference2(&container));
}

// A phantom type parameter is one that doesn't show up at runtime, but is checked statically (and only) at compile time.
// Data types can use extra generic type parameters to act as markers or to perform type checking at compile time. 
// These extra parameters hold no storage values, and have no runtime behavior.
// PhantomData<T>表示PhantomData类型, 而PhantomData表示该类型的值。不占用存储空间，只在编译时对类型等进行检查
use std::marker::PhantomData;

#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);


// Note: Storage is allocated for generic type `A`, but not for `B`.
//       Therefore, `B` cannot be used in computations.
fn phantom_data() {
	println!("~~~~~~~phantom data~~~~~~~");
	
	let _t1: PhantomTuple<char, i32> = PhantomTuple('Q', PhantomData);
	let _t2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);
	
	// error[E0308]: mismatched types
	// note: expected struct `PhantomTuple<_, i32>`
    //          found struct `PhantomTuple<_, f64>`
	//println!("_t1 == _t2 yields: {}", _t1 == _t2);
}

use std::ops::Add;
#[derive(Debug, Clone, Copy)]
enum Inch {}

#[derive(Debug, Clone, Copy)]
enum Mm {}

#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
	type Output = Length<Unit>;
	
	fn add(self, rhs: Self::Output) -> Self::Output {
		Length(self.0 + rhs.0, PhantomData)
	}
}

fn phantom_use() {
	println!("~~~~~~~Phantom use~~~~~~~");
	
	let one_foot: Length<Inch> = Length(12.0, PhantomData);
	let one_meter: Length<Mm> = Length(1000.0, PhantomData);
	
	let two_feet = one_foot + one_foot;
	let two_meters = one_meter + one_meter;
	
	println!("one foot + one_foot = {:?} in", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);
	
	// can't add with difference Unit
	// error[E0308]: mismatched types
	//let one_feter = one_foot + one_meter;
}

fn main() {
	functions();
	impl_func();
	impl_trait();
	
	bound_trait();
	bound_empty_trait();
	bound_multi_trait();
	bound_where_clauses();
	
	without_associated_types();
	associated_types();
	
	phantom_data();
	phantom_use();
}