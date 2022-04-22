use std::fmt;

//TODO: convert to template
struct Pair(Box<u32>, Box<u32>);

impl Pair {
	fn new(first: u32, second: u32) -> Pair {
		Pair(Box::new(first), Box::new(second))
	}
	
	fn destroy(self: Self) {
		let Pair(first, second) = self;
		
		println!("Destroying pair({:?}, {:?})", first, second);
	}
}

impl fmt::Display for Pair {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "first: {:?}, second: {:?}", self.0, self.1)
	}
}

fn method() {
	println!("~~~~~~~method~~~~~~~");
	let pair = Pair::new(5, 7);
	println!("{}", pair);
	
	pair.destroy();
	
	// ^^^^ value borrowed here after move
	//println!("Pair: {:?}", pair);
}

/*
Closures are functions that can capture the enclosing environment.
	using || instead of () around input variables.
	optional body delimination ({}) for a single expression (mandatory otherwise).
	the ability to capture the outer environment variables.
*/
fn closure_simple() {
	println!("~~~~~~~closure_simple~~~~~~~");
	fn function(i: i32) -> i32 { i + 1 }
	
	let closure_annotated = |i: i32| -> i32 { i + 1 };
	let closure_inferred  = |i	   |           i + 1 ;
	
	let i = 1;
	println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));
	
	let one = || 1;
	println!("closure return one: {}", one());
}

/*
Closures can capture variables:
	by reference: &T
	by mutable reference: &mut T
	by value: T
*/
fn closure_capturing() {
	use std::mem;	
	println!("~~~~~~~closure_capturing~~~~~~~");
	
	let color = String::from("Green");
	// A closure to print `color` which immediately borrows (`&`) `color` and
    // stores the borrow and closure in the `print` variable. It will remain
    // borrowed until `print` is used the last time.
	let print = || println!("'Color': {}", color);
	print();
	
	// `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`.
	let _reborrow = &color;
	print();
	
	// A move or reborrow is allowed after the final use of `print`
	let _color_move = color;
	// ^^^^^ move out of `color` occurs here
	//print();
	
	let mut count = 0;
	// A closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately
    // borrows `count`.
	
	// A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates the closure which requires a `mut`.
	let mut inc = || {
	// error[E0596]: cannot borrow `inc` as mutable, as it is not declared as mutable
	//let inc = || {		
		count += 1;
		println!("'count': {}", count);
	};
	inc();
	
	// The closure still mutably borrows `count` because it is called later.
    // An attempt to reborrow will lead to an error.
	// error[E0502]: cannot borrow `count` as immutable because it is also borrowed as mutable
    //let _reborrow = &count;
	inc();
	
	// The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let ref mut _count_reborrowed = count; // equals to 'let _count_reborrowed = &mut count;'
	// error[E0499]: cannot borrow `count` as mutable more than once at a time
	//inc();
	
	//A non-copy type
	let noncopyable = Box::new(5);
	// `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `noncopyable` immediately moves into
    // the closure.
	let consume = || {
		println!("'noncopyable': {}", noncopyable);
		mem::drop(noncopyable);
	};
	
	consume();
	// error[E0382]: use of moved value: `consume`
	//consume();
	
	//Using move before vertical pipes forces closure to take ownership of captured variables
	// `Vec` has non-copy semantics.
	let haystack = vec![1, 2, 3];
	println!("haystack: {:?}", haystack);
	let contains = move |needle| haystack.contains(needle);
	
	println!("haystack contains 1: {}", contains(&1));
	println!("haystack contains 4: {}", contains(&4));
	
	// error[E0382]: borrow of moved value: `haystack`
	//println!("There're {} elements in vec", haystack.len());
	
	// Removing `move` from closure's signature will cause closure
    // to borrow _haystack_ variable immutably, hence _haystack_ is still
    // available and uncommenting above line will not cause an error.
}

/*
When taking a closure as an input parameter, the closure's complete type must be annotated using one of a few traits. they are:

	Fn: the closure captures by reference (&T)
	FnMut: the closure captures by mutable reference (&mut T)
	FnOnce: the closure captures by value (T)
	
On a variable-by-variable basis, the compiler will capture variables in the least restrictive manner possible.

For instance, consider a parameter annotated as FnOnce. 
This specifies that the closure may capture by &T, &mut T, or T, 
but the compiler will ultimately choose based on how the captured variables are used in the closure.

This is because if a move is possible, then any type of borrow should also be possible. 
Note that the reverse is not true. 
If the parameter is annotated as Fn, then capturing variables by &mut T or T are not allowed.
*/
fn apply<F>(f: F) where
	F: FnOnce() {
	
	f();
}

fn apply_to_3<F>(f: F) -> i32 where
	F: Fn(i32) -> i32 {
	
	f(3)
}

fn closure_as_input() {
	use std::mem;
	println!("~~~~~~~closure_as_input~~~~~~~");
	
	let greeting = "hello";
	let mut farewell: String = "goodbye".to_owned(); // equals to '= String::from("goodbye");'
	
	// Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
	let diary = || {
		// `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);
		
		// Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
		farewell.push_str("!!!");
		println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");
		
		// Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
		mem::drop(farewell);
	};
	
	apply(diary);
	
	let double = |x| 2 * x;
	println!("3 doubled: {}", apply_to_3(double));
}

/*
When a closure is defined, the compiler implicitly creates a new anonymous structure to store the captured variables inside, 
meanwhile implementing the functionality via one of the traits: Fn, FnMut, or FnOnce for this unknown type.
This type is assigned to the variable which is stored until calling.

the Fn, FnMut, and FnOnce traits dictate how a closure captures variables from the enclosing scope.
*/
fn call_me<F: Fn()>(f: F) {
	f();
}

fn function() {
	println!("I'm a function");
}

fn fn_as_input() {
	println!("~~~~~~~fn_as_input~~~~~~~");
	
	let closure = || println!("I'm a closure");
	
	call_me(closure);
	call_me(function);
}

/*
The valid traits for returning a closure are: Fn, FnMut, FnOnce
Beyond this, the move keyword must be used, which signals that all captures occur by value. 
This is required because any captures by reference would be dropped as soon as the function exited, 
leaving invalid references in the closure.
*/
fn create_fn() -> impl Fn() {
	let text = "Fn".to_owned();
	
	move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
	let text = "FnMut".to_owned();
	
	move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
	let text = "FnOnce".to_owned();
	
	move || println!("This is a: {}", text)
}

fn closure_as_output() {
	println!("~~~~~~~closure_as_output~~~~~~~");
	let fn_plain = create_fn();
	let mut fn_mut = create_fnmut();
	let fn_once = create_fnonce();
	
	fn_plain();
	fn_mut();
	fn_once();
}

/*
pub trait Iterator {
    // The type being iterated over.
    type Item;

    // `any` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.
    fn any<F>(&mut self, f: F) -> bool where
        // `FnMut` meaning any captured variable may at most be
        // modified, not consumed. `Self::Item` states it takes
        // arguments to the closure by value.
        F: FnMut(Self::Item) -> bool {}
}
*/
// Destructuring主要使用在match匹配，closure参数匹配等
// 参考：05_flow_control.rs - match_pointer_ref()及以下例子
fn closure_iter_any() {
	println!("~~~~~~~closure_iter_any~~~~~~~");
	let vec1 = vec![1, 2, 3];
	let vec2 = vec![4, 5, 6];
	
	println!("2 in vec1: {}", vec1.iter().any(|&v| v == 2));
	// `into_iter()` for vecs yields `i32`. No destructuring required.
	println!("2 in vec2: {}", vec2.into_iter().any(|v| v == 2));
	
	let arr1 = [1, 2, 3];
	let arr2 = [4, 5, 6];
	println!("2 in arr1: {}", arr1.iter().any(|&v| v == 2));
	// `into_iter()` for arrays unusually yields `&i32`.
	// error[E0277]: can't compare `&{integer}` with `{integer}`
	//println!("2 in arr2: {}", arr2.into_iter().any(|v| v == 2));
	println!("2 in arr2: {}", arr2.into_iter().any(|&v| v == 2));
}

/*
pub trait Iterator {
    // The type being iterated over.
    type Item;

    // `find` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
        // `FnMut` meaning any captured variable may at most be
        // modified, not consumed. `&Self::Item` states it takes
        // arguments to the closure by reference.
        P: FnMut(&Self::Item) -> bool {}
}
*/
fn closure_iter_find() {
	println!("~~~~~~~closure_iter_find~~~~~~~");
	let vec1 = vec![1, 2, 3];
	let vec2 = vec![4, 5, 6];
	
	let mut iter = vec1.iter();
	let mut into_iter = vec2.into_iter();
	
	// `iter()` for vecs yields `&i32`, and we want to reference one of its
    // items, so we have to destructure `&&i32` to `i32`
	println!("Find 2 in vec1: {:?}", iter.find(|&&v| v == 2));	
	// `into_iter()` for vecs yields `i32`, and we want to reference one of
    // its items, so we have to destructure `&i32` to `i32`
	println!("Find 2 in vec2: {:?}", into_iter.find(|&v| v == 2));
	
	let arr1 = [1, 2, 3];
	let arr2 = [4, 5, 6];
	println!("Find 2 in arr1: {:?}", arr1.iter().find(|&&v| v == 2));
	// `into_iter()` for arrays unusually yields `&i32`	
	println!("Find 2 in arr2: {:?}", arr2.into_iter().find(|&&v| v == 2));
}

// Rust provides Higher Order Functions (HOF). 
// These are functions that take one or more functions and/or produce a more useful function. 
// HOFs and lazy iterators give Rust its functional flavor.
fn is_odd(n: u32) -> bool {
	n % 2 == 1
}

fn hof() {
	println!("~~~~~~~Higher Order Functions~~~~~~~");
	let range: std::ops::Range<u32> = 1..10;
	let range_inc: std::ops::RangeInclusive<u32> = 1..=10;
	println!("range: {:?}", range);
	println!("range_inclusive: {:?}", range_inc);
	
	let print = |x| print!("{} ", x);
	range.for_each(print);
	println!("");
	range_inc.for_each(print);
	println!("");
	
	println!("Find the sum of all the squared odd numbers under 1000");
	const UPPER: u32 = 1000;
	let mut acc = 0;
	for n in 0.. {
		let n_squared = n * n;
		if n_squared >= UPPER {
			break;
		} else if is_odd(n_squared) {
			acc += n_squared;
		}
	}
	println!("imperative style: {}", acc);
	
	let sum_of_squared_odd_numbers: u32 = 
		(0..).map(|n| n * n)
			 .take_while(|&n_squared| n_squared < UPPER)
			 .filter(|&n_squared| is_odd(n_squared))
			 .fold(0, |acc, n_squared| acc + n_squared);
	println!("functional style: {}", sum_of_squared_odd_numbers);
}

/*
Diverging functions never return. They are marked using !, which is an empty type.
The main advantage of this type is that it can be cast to any other one and therefore used at places where an exact type is required, for instance in match branches.
It is also the return type of functions that loop forever (e.g. loop {}) like network servers or functions that terminate the process (e.g. exit()).
*/
//#![feature(never_type)]
fn never_return() -> ! {
	//let x: ! = panic!("This call never returns.");    
	
	panic!("The call never return");
	println!("You will never see this line!");
}

fn sum_odd_numbers(up_to: u32) -> u32 {
	let mut acc = 0;
	for n in 0..up_to {
		// Notice that the return type of this match expression must be u32
        // because of the type of the "addition" variable.
		let addition: u32 = match n % 2 == 1 {
			true => n,
			// On the other hand, the "continue" expression does not return
            // u32, but it is still fine, because it never returns and therefore
            // does not violate the type requirements of the match expression.
			false => continue,
		};
		
		acc += addition;
	}
	
	acc
}

fn sum_odd_numbers_functional(up_to: u32) -> u32 {
	(0..up_to)
		.filter(|&n| n % 2 == 1)
		.fold(0, |acc, n| acc + n)
		
}

fn main() {
	method();
	
	closure_simple();
	closure_capturing();
	closure_as_input();
	fn_as_input();
	closure_as_output();
	closure_iter_any();
	closure_iter_find();

	hof();
	println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
	println!("functional Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers_functional(9));
	never_return();
}