/*
macros look like functions, except that their name ends with a bang !, 
but instead of generating a function call, macros are expanded into source code that gets compiled with the rest of the program. 

So why are macros useful?
	Don't repeat yourself.
	Domain-specific languages.
	Variadic interfaces. (Sometimes you want to define an interface that takes a variable number of arguments)
*/

macro_rules! say_hello {
	
	// `()` indicates that the macro takes no argument.
	() => {
		// The macro will expand into the contents of this block.
		println!("Hello!");
	};
}

fn macro_simple() {
	println!("~~~~~~~macro simple~~~~~~~");
	say_hello!();
}

// The arguments of a macro are prefixed by a dollar sign $ and type annotated with a designator
// valid fragment specifiers are `ident`, `block`, `stmt`, `expr`, `pat`, `ty`, `lifetime`, `literal`, `path`, `meta`, `tt`, `item` and `vis`
// macro ref: https://doc.rust-lang.org/reference/macros-by-example.html
macro_rules! create_function {
	($func_name: ident) => {
		fn $func_name() {
			println!("You called {:?}()", stringify!($func_name));
		}
	};
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
	($expression: expr) => {
		println!("{:?} = {:?}", stringify!($expression), $expression);
	};
}

fn macro_designators() {
	println!("~~~~~~~macro designators~~~~~~~");
	
	foo();
	bar();
	
	print_result!(1u32 + 1);
	print_result!({
		let x = 1u32;
		x * x + 2 * x - 1
	});
}

macro_rules! test {
	($left: expr; and $right: expr) => {
		println!("{:?} and {:?} is {:?}",
			stringify!($left), stringify!($right), $left && $right);
	};
	($left: expr; or $right: expr) => {
		println!("{:?} and {:?} is {:?}",
			stringify!($left), stringify!($right), $left || $right);
	};
}

fn macro_overload() {
	println!("~~~~~~~macro overload~~~~~~~");
	
	test!(1i32+1 == 2i32; and 2i32*2 == 4i32);
	test!(true; or false);
}

// Macros can use + in the argument list to indicate that an argument may repeat at least once, 
// or *, to indicate that the argument may repeat zero or more times.
macro_rules! find_min {
	($x: expr) => {$x};
	($x: expr, $($y: expr),+) => {
		std::cmp::min($x, find_min!($($y),+))
	};
}

fn macro_repeat() {
	println!("~~~~~~~macro repeat~~~~~~~");
	
	println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2, 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
}

fn main() {
	macro_simple();
	
	macro_designators();
	macro_overload();
	macro_repeat();
}