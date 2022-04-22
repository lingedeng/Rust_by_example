//#![allow(unused_labels)]
//#![allow(unreachable_code)]
//#![allow(unused_variables)]

fn if_else() {
	println!("~~~~~~~if_else~~~~~~~");
	
	let n = 5;
	if n > 0 {
		print!("{} is positive", n);
	} else if n < 0 {
		print!("{} is negative", n);
	} else {
		print!("{} is zero", n);
	}
	
	let big_n = if n < 10 || n > -10 {
		println!(", and is a small number, increase ten-fold");
		10 * n
	} else {
		println!(", and is a big number, halve the number");
		n / 2
	};
		
	println!("{} -> {}", n, big_n);
}

fn loop_1() {
	println!("~~~~~~~loop_1~~~~~~~");
	let mut count = 0u32;
	println!("Let's count until infinity!");
	
	loop {
		count += 1;
		
		if count == 3 {
			println!("Three!");
			continue;
		}
		
		println!("{}", count);
		
		if count == 5 {
			println!("Ok, enough");
			break;
		}
	}
}


fn loop_2() {
	println!("~~~~~~~loop_2~~~~~~~");
	'outer: loop {
		println!("Enter outer loop");
		
		'inner: loop {
			println!("Enter inner loop");
			
			// This would break only the inner loop
			//break;
			
			// This breaks the outer loop
			break 'outer;
		}
		
		println!("Never print out");
	}
	
	println!("Exit outer loop");
}

fn loop_3() {
	println!("~~~~~~~loop_3~~~~~~~");
	let mut counter = 0;
	
	let result = loop {
		counter += 1;
		
		if counter == 10 {
			break counter * 2;
		}
	};
	
	assert_eq!(result, 20);
	println!("{} -> {}", counter, result);
}

fn bizzbuzz(n: i32) {
	if n % 15 == 0 {
		println!("bizzbuzz");
	} else if n % 3 == 0 {
		println!("bizz");
	} else if n % 5 == 0 {
		println!("buzz");
	} else {
		println!("{}", n);
	}
}

fn while_loop() {
	println!("~~~~~~~while_loop~~~~~~~");
	let mut n = 1;
	
	while n < 18 {
		bizzbuzz(n);
		n += 1;
	}
}

fn for_range() {
	println!("~~~~~~~for_range~~~~~~~");
	// create an iterator is to use the range notation a..b
	//for n in 1..18 {
	for n in 1..=18 {
		bizzbuzz(n);
	}
}

fn for_iterator() {
	println!("~~~~~~~for_iterator~~~~~~~");
	let vec1: Vec<&str> = vec!["Bob", "Chen", "Eric"];
	let vec2 = vec!["Bob", "Chen", "Eric"];
	let mut vec3 = vec!["Bob", "Chen", "Eric"];
	
	println!("***iter()***");
	// This borrows each element of the collection through each iteration. 
	// Thus leaving the collection untouched and available for reuse after the loop.	
	for name in vec1.iter() { // name type is &&str
		match name {
			// ^^^^^^ expected `&str`, found `str`
			//"Chen" => println!("There is a rustacean among us!"),
			&"Chen" => println!("There is a rustacean among us!"),
			_ => println!("Hello {}!", name),
		}
	}
	println!("names: {:?}", vec1);
	
	println!("***into_iter()***");
	// This consumes the collection so that on each iteration the exact data is provided. 
	// Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.
	for name in vec2.into_iter() { // name type is &str
		match name {
			"Chen" => println!("There is a rustacean among us!"),
			_ => println!("Hello {}!", name),
		}
	}
	//println!("names: {:?}", vec2);
	
	println!("***iter_mut()***");
	println!("before names: {:?}", vec3);
	// This mutably borrows each element of the collection, allowing for the collection to be modified in place.
	for name in vec3.iter_mut() { // name type is &mut &str
		*name = match name {
			&mut"Chen" => "There is a rustacean among us!",
			_ => "Hello",
		}
	}
	println!("after  names: {:?}", vec3);
}

fn match_1() {
	println!("~~~~~~~match_1~~~~~~~");
	let num = 11;
	print!("Tell me about {}, ", num);
	
	match num {
		1 => println!("One"),
		2 | 3 | 5 | 7 | 11 => println!("Prime"),
		13..=19 => println!("The teen"),
		_ => println!("Ain't special"),
	}
	
	let boolean = true;
	let result = match boolean {
		true => 1,
		false => 0,
	};
	println!("{} -> {}", boolean, result);
}

fn match_tuple() {
	println!("~~~~~~~match_tuple~~~~~~~");
	let triples = (1, -2, 3);
	
	println!("{:?}", triples);
	match triples {
		(0, y, z) => println!("first is 0, y is {}, z is {}", y, z),
		// `..` can be used to ignore the rest of the tuple
		(1, ..) => println!("first is 1 and the rest doesn't matter"),
		// `_` means don't bind the value to a variable
		_ => println!("It doesn't matter what they are"),
	}
}

#[allow(dead_code)]
enum Color {
    // These 3 are specified solely by their name.
    Red,
    Blue,
    Green,
    // These likewise tie `u32` tuples to different names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn match_enum() {
	println!("~~~~~~~match_enum~~~~~~~");
	let color = Color::RGB(122, 17, 40);    

    println!("What color is it?");
    // An `enum` can be destructured using a `match`.
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k),
        // Don't need another arm because all variants have been examined
    }
}

fn match_struct() {
	struct Foo {
		x: (u32, u32),
		y: u32,
	}
	
	println!("~~~~~~~match_struct~~~~~~~");
	
	let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y),
		// this will give an error: error[E0025]: field `_` bound multiple times in the pattern
		//Foo {_, _} => println!("we don't care struct"),
		
		_ => println!("we don't care struct"),		
    }
}

/*
For pointers, a distinction needs to be made between destructuring and dereferencing as they are different concepts which are used differently from a language like C.

Dereferencing uses *
Destructuring uses &, ref, and ref mut
*/
fn match_pointer_ref() {
	println!("~~~~~~~match_pointer_ref~~~~~~~");
	let reference: &i32 = &4;
	
	match reference {
		// If `reference` is pattern matched against `&val`, it results
        // in a comparison like:
        // `&i32`
        // `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
		&val => println!("Got a value via destructuring: {}", val),
	}
	
	match *reference {
		val => println!("Got a value via destructuring: {}", val),
	}
	
	let not_a_reference = 3;
	// Rust provides `ref` for exactly this purpose. It modifies the
    // assignment so that a reference is created for the element; this
    // reference is assigned.
	let ref is_a_reference = 3;
	println!("{} - {}", not_a_reference, is_a_reference);
	
	let value = 5;
	let mut mut_value = 6;
	
	// Use `ref` keyword to create a reference.
	match value {
		//ref r => println!("Got a reference to a value: {}", r), // auto Dereferencing
		ref r => println!("Got a reference to a value: {}", *r),
	}
	
	match mut_value {
		ref mut m => {
			// error[E0368]: binary assignment operation `+=` cannot be applied to type `&mut {integer}`
			//m += 10; //no auto dereferencing
			*m += 10;
			println!("We added 10. `mut_value`: {:?}", m);
		},
	}
}

fn match_guard() {
	println!("~~~~~~~match_guard~~~~~~~");
	let twins = (2, -2);
	
	match twins {
		// The ^ `if condition` part is a guard
		(x, y) if x == y => println!("These are twins"),
		(x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
		(x, _) if x % 2 == 1 => println!("First one is odd!"),
		// comment next line, report: error[E0004]: non-exhaustive patterns: `(_, _)` not covered
		//_ => println!("No correlation..."),
		(_, _) => println!("No correlation..."),
	}
	
	let number = -4i8;
	match number {
		i if i == 0 => println!("Zero"),
		i if i > 0 => println!("Greater than zero"),		
		//i => println!("Not zero"),
		_ => println!("Fell through"),
	}
	
	let age = 15;
	match age {
		n if n == 0 => println!("I haven't celebrated my first birthday yet"),
		n if n >= 1 && n <= 12 => println!("I'm a child of age {:?}", n),
		n if n >= 13 && n <= 19 => println!("I'm a teen of age {:?}", n),
		n => println!("I'm an old person of age {:?}", n),
	}
}

fn age() -> u32 {
	15
}

fn some_number() -> Option<u32> {
	Some(42)
}

fn match_binding() {
	println!("~~~~~~~match_binding~~~~~~~");
	match age() {
		0 => println!("I haven't celebrated my first birthday yet"),
		n @ 1..=12 => println!("I'm a child of age {:?}", n),
		n @ 13..=19 => println!("I'm a teen of age {:?}", n),
		n => println!("I'm an old person of age {:?}", n),
	}
	
	match some_number() {
		Some(n @ 42) => println!("The answer is {}", n),
		Some(n) => println!("Not interesting... {}", n),
		_ => println!("No value"),
	}
}

enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn if_let() {
	println!("~~~~~~~if_let~~~~~~~");
	
	let number = Some(7);
	let letter: Option<i32> = None;
	let emoticon: Option<i32> = None;
	
	if let Some(i) = number {
		println!("Matched {}", i);
	}
	
	if let Some(i) = letter {
		println!("Matched {}", i);
	} else {
		println!("Didn't match a number. Let's go with a letter!");
	}
	
	let i_like_letters = false;
	if let Some(i) = emoticon {
		println!("Matched {}", i);
	} else if i_like_letters {
		println!("Didn't match a number. Let's go with a letter!");
	} else {
		println!("I don't like letters. Let's go with an emoticon :)!");
	}
	
	let a = Foo::Bar;
	let b = Foo::Baz;
	let c = Foo::Qux(100);
	
	if let Foo::Bar = a {
		println!("a is foobar");
	}
	
	if let Foo::Bar = b {
		println!("b is foobar");
	}
	
	if let Foo::Qux(i) = c {
		println!("c is {}", i);
	}
	
	if let Foo::Qux(i @ 100) = c {
		println!("c is one hundred");
	}
}

fn while_let() {
	println!("~~~~~~~while_let~~~~~~~");
	
	let mut optional = Some(0);
	
	while let Some(i) = optional {
		if i > 9 {
			println!("Greater than 9");
			optional = None;
		} else {
			println!("i is {}, try again", i);
			optional = Some(i+1);
		}		
	}
	// ^ `if let` had additional optional `else`/`else if`
    // clauses. `while let` does not have these.
}

fn main() {	
	if_else();
	
	loop_1();
	loop_2();
	loop_3();
	
	while_loop();
	
	for_range();
	for_iterator();
	
	match_1();
	match_tuple();
	match_enum();
	match_struct();
	match_pointer_ref();
	match_guard();
	match_binding();
	
	if_let();
	while_let();
}