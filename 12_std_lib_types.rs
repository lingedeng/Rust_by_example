use std::mem;

fn print_knowledage_point(s: &str) {
	println!("~~~~~~~{}~~~~~~~", s);
}

#[allow(dead_code)]
struct Point {
	x: f64,
	y: f64,
}

#[allow(dead_code)]
struct Rectangle {
	top_left: Point,
	bottom_right: Point,
	// slice present by wide pointer (fat pointer), pointer + length(length type: usize)
	content: &'static str,
}

fn origin() -> Point {
	Point {x: 0.0, y: 0.0}
}

fn add_one(x: usize) -> usize {
	x + 1
}

use std::num::ParseIntError;

fn boxed_type() {
	print_knowledage_point("boxed type");
	
	let point = origin();
	let rect = Rectangle {
		top_left: origin(),
		bottom_right: Point { x: 3.0, y: -4.0 },
		content: "whoami",
	};	
	
	let boxed_point = Box::new(origin());
	let box_in_a_box = Box::new(Box::new(origin()));
	
	println!("Point occupies {} bytes on the stack",
             mem::size_of_val(&point));
    println!("Rectangle occupies {} bytes on the stack",
             mem::size_of_val(&rect));

    // box size == pointer size
    println!("Boxed point occupies {} bytes on the stack",
             mem::size_of_val(&boxed_point));    
    println!("Boxed box occupies {} bytes on the stack",
             mem::size_of_val(&box_in_a_box));
			 
	let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes on the stack",
             mem::size_of_val(&unboxed_point));
			 
	// a reference is just a pointer that is assumed to be aligned, not null, and pointing to memory containing a valid value of T
	// Option<&T> has the same memory representation as a nullable but aligned pointer, and can be passed across FFI boundaries as such.
	// the comparison operators transparently defer to the referent’s implementation, allowing references to be compared the same as owned values.
	// &mut T references can be freely coerced into &T references with the same referent type, and references with longer lifetimes can be freely coerced into references with shorter ones.
	// instead of comparing the values pointed to, is accomplished via implicit reference-pointer coercion and raw pointer equality via ptr::eq, while PartialEq compares values.
	let a = &1;
	println!("Ref occupies {} bytes on the stack",
		mem::size_of_val(&a));
		
	let hello = "hello, world";
	assert_eq!(16, mem::size_of_val(&hello));
	println!("str ref occupies {} bytes on the stack",
		mem::size_of_val(&hello));
	
	/*
	A vector is represented using 3 parameters:
		pointer to the data
		length
		capacity
	*/
	let vec_point: Vec<Point> = vec![];
	let vec_int = vec![1, 2, 3];
	let boxed_vec = Box::new(vec!["1", "2", "3"]);
	
	println!("Point vec occupies {} bytes on the stack",
             mem::size_of_val(&vec_point));
    println!("Integer vec occupies {} bytes on the stack",
             mem::size_of_val(&vec_int));
	println!("Boxed vec occupies {} bytes on the stack",
             mem::size_of_val(&boxed_vec));
	
	// A String is stored as a vector of bytes
	let string = String::from("whoami");
	println!("String occupies {} bytes on the stack",
             mem::size_of_val(&string));
	
	// error[E0277]: the size for values of type `[i32]` cannot be known at compilation time
	//let vec_slice: [i32] = vec_int[..2];
	// error[E0308]: mismatched types 
	// ^^^^^^^^^^^^^ expected slice `[i32]`, found `&[{integer}]`
	//let vec_slice: [i32] = &vec_int[..2];
	let vec_slice: &[i32] = &vec_int[..2];
	println!("Slice vec occupies {} bytes on the stack",
             mem::size_of_val(&vec_slice));
			 
	let some = Some(7);
	let none: Option<i32> = None;
	println !("Some occupies {} bytes on the stack",
		mem::size_of_val(&some));
	println !("None occupies {} bytes on the stack",
		mem::size_of_val(&none));
	
	
	let ok: Result<i32, ParseIntError> = Ok(7);
	let err: Result<i64, &'static str> = Err("invalid value");
	println !("Ok occupies {} bytes on the stack",
		mem::size_of_val(&ok));
	println !("Err occupies {} bytes on the stack",
		mem::size_of_val(&err));
		
	let contacts: std::collections::HashMap<&'static str, &'static str> = HashMap::new();
	println !("Hashmap occupies {} bytes on the stack",
		mem::size_of_val(&contacts));
	
	// Function pointers are pointers that point to code, not data.
	let ptr: fn(usize) -> usize = add_one;
	println !("fn ptr for normal function occupies {} bytes on the stack",
		mem::size_of_val(&ptr));
		
	let clos: fn(usize) -> usize = |x| x + 5;
	println !("fn ptr for closure occupies {} bytes on the stack",
		mem::size_of_val(&clos));
	
}

fn str_literals_and_escapes() {
	print_knowledage_point("string literals and escapes");
	
	let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // ...or Unicode code points.
    let unicode_codepoint = "\u{2190}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+2190) is called {}",
                unicode_codepoint, character_name );
	
	println!("~without escapes~");
	let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // If you need quotes in a raw string, add a pair of #s
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // If you need "# in your string, just use more #s in the delimiter.
    // There is no limit for the number of #s you can use.
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
	
	println!("~Byte strings~");
	// Remember, str and String must be valid UTF-8
	// Note that this is not actually a `&str`
    let bytestring: &[u8; 21] = b"this is a byte string";

    // Byte arrays don't have the `Display` trait, so printing them is a bit limited
    println!("A byte string: {:?}", bytestring);

    // Byte strings can have byte escapes...
    let escaped = b"\x52\x75\x73\x74 as bytes";
    // ...but no unicode escapes
    // let escaped = b"\u{211D} is not allowed";
    println!("Some escaped bytes: {:?}", escaped);


    // Raw byte strings work just like raw strings
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    // Converting a byte array to `str` can fail
    if let Ok(my_str) = std::str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }

    let _quotes = br#"You can also use "fancier" formatting, \
                    like with normal raw strings"#;

    // Byte strings don't have to be UTF-8
	// error[E0308]: mismatched types
	// expected struct `String`, found `&[u8; 9]`
    //let chinese: String = b"\xe6\x88\x91\xe6\x98\xaf\xe8\xb0\x81"; // "who am i" in chinese
	let chinese = b"\xe6\x88\x91\xe6\x98\xaf\xe8\xb0\x81"; // "who am i" in chinese

    // But then they can't always be converted to `str`
    match std::str::from_utf8(chinese) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    };
}

use std::num::ParseFloatError;

enum MathError {
	ParseError(ParseFloatError),
	DivisionByZero,	
}

type MathResult = Result<f64, MathError>;

impl From<ParseFloatError> for MathError {
	fn from(error: ParseFloatError) -> Self {
		MathError::ParseError(error)
	}
}

fn div(x: &'static str, y: &'static str) -> MathResult {
	let n1 = x.parse::<f64>()?;
	let n2 = y.parse::<f64>()?;
	
	if n2 == 0.0 {
		Err(MathError::DivisionByZero)
	} else {
		Ok(n1 / n2)
	}
}

fn op_(x: &'static str, y: &'static str) -> MathResult {
	div(x, y)
}

fn op(x: &'static str, y: &'static str) {
	match op_(x, y) {
		Err(why) => match why {
			MathError::ParseError(err) => println!("{}", err),
			MathError::DivisionByZero => println!("division by zero"),
		},
		Ok(value) => println!("div value: {}", value),
	}
}

fn error_question_mark() {
	print_knowledage_point("error question mark");
	
	op("1.0", "0.0");
}

use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798-1364" => "We're sorry, the call cannot be completed as dialed. 
            Please hang up and try again.",
        "645-7689" => "Hello, this is Mr. Awesome's Pizza. My name is Fred.
            What can I get for you today?",
        _ => "Hi! Who is this again?"
    }
}

fn hashmap_example() {
	print_knowledage_point("Hashmap example");
	
	let mut contacts: HashMap<String, &'static str> = HashMap::new();
	
	contacts.insert(String::from("Daniel"), "798-1364");
	contacts.insert(String::from("Ashley"), "645-7689");
    contacts.insert(String::from("Katie"), "435-8291");
    contacts.insert(String::from("Robert"), "956-1745");
	
	match contacts.get("Daniel") {
		Some(&number) => println!("Calling Daniel: {}", call(number)),
        _ => println!("Don't have Daniel's number.")
	}
	
	// `HashMap::insert()` returns `None`
    // if the inserted value is new, `Some(value)` otherwise
	match contacts.insert(String::from("Daniel"), "164-6743") {
		None => println!("insert Daniel successed"),
		Some(number) => println!("Daniel exist, old number: {}", number),
	}
	
	match contacts.get("Ashley") {
		Some(&number) => println!("Calling Daniel: {}", call(number)),
        _ => println!("Don't have Daniel's number.")
	}
	
	match contacts.remove(&String::from("Ashley")) {
		Some(number) => println!("Ashley exist, old number: {}", number),
		None => println!("Ashley not exist"),
	}
	
	match contacts.remove(&String::from("eric")) {
		Some(number) => println!("eric exist, old number: {}", number),
		None => println!("eric not exist"),
	}
	
	// `HashMap::iter()` returns an iterator that yields 
    // (&'a key, &'a value) pairs in arbitrary order.
	for (contact, number) in contacts.iter() {
		println!("Calling {}: {}", contact, call(number));
	}
	
}

#[derive(PartialEq, Eq, Hash)]
struct Account<'a> {
	user_name: &'a str,
	pwd: &'a str,
}

struct AccountInfo<'a> {
	name: &'a str,
	email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_login<'a>(accounts: &Accounts<'a>,
	username: &'a str, password: &'a str) {

	println!("Username: {}, password: {}, try login...", username, password);
	let account = Account {
		user_name: username,
		pwd: password,
	};
	
	match accounts.get(&account) {
		Some(acct_info) => {
			println!("login success");
			println!("name: {}, email: {}", acct_info.name, acct_info.email);
		},
		None => println!("login failed!"),
	}
}

fn hashmap_custom_key() {
	print_knowledage_point("Hashmap custom key");
	
	let account = Account {
		user_name: "whoami",
		pwd: "123456",
	};
	
	let acct_info = AccountInfo {
		name: "eric",
		email: "whoami@163.com",
	};
	
	let mut accounts = Accounts::new();
	accounts.insert(account, acct_info);
	
	try_login(&accounts, "whoami", "123.com");
	try_login(&accounts, "whoami", "123456");
}

use std::rc::Rc;

fn smart_point_rc() {
	print_knowledage_point("smart point rc");
	
	let rc_example = String::from("whoami");
	let rc_int = 1;
	{
		println!("--- rc_a is created ---");
		let rc_a = Rc::new(rc_example);
		
		println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
		
		let rc_int_a = Rc::new(rc_int);
		println!("Reference Count of rc_int_a: {}", Rc::strong_count(&rc_int_a));
		
		{
			println!("--- rc_a is cloned to rc_b ---");
            
            let rc_b: Rc<String> = Rc::clone(&rc_a);
            println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
			
			// Two `Rc`s are equal if their inner values are equal
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));
            
            // We can use methods of a value directly
            println!("Length of the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b);
            
            println!("--- rc_b is dropped out of scope ---");
		}
		
		println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
        
        println!("--- rc_a is dropped out of scope ---");
	}
	
	// Error! `rc_examples` already moved into `rc_a`
    // And when `rc_a` is dropped, `rc_examples` is dropped together
	//println!("rc_example: {}", rc_example);
	println!("rc_int: {}", rc_int);
}

use std::sync::Arc;
use std::thread;

fn smart_point_arc() {
	print_knowledage_point("Smart point Arc");
	
	let apple = Arc::new("the same apple");
	
	for _ in 0..10 {
		let apple = Arc::clone(&apple);
		
		thread::spawn(move || {
			println!("{:?}", apple);
		});
	}
}

fn main() {
	boxed_type();
	
	str_literals_and_escapes();
	
	error_question_mark();
	
	hashmap_example();
	hashmap_custom_key();
	
	smart_point_rc();
	smart_point_arc();
}