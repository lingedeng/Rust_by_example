#![allow(overflowing_literals)]
#![allow(dead_code)]

fn casting() {
	println!("~~~~~~~casting~~~~~~~");
    let decimal = 65.4321_f32;

    // Error! No implicit conversion
    //let integer: u8 = decimal;
    // FIXME ^ Comment out this line

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    // Error! There are limitations in conversion rules. 
    // A float cannot be directly converted to a char.
    //let character = decimal as char;
    // FIXME ^ Comment out this line

	assert_eq!(decimal, 65.4321_f32);
	assert_eq!(integer, 65u8);
	assert_eq!(character, 'A');
    println!("Casting: {} -> {} -> {}", decimal, integer, character);	

    // when casting any value to an unsigned type, T,
    // T::MAX + 1 is added or subtracted until the value
    // fits into the new type

    // 1000 already fits in a u16
	assert_eq!(1000 as u16, 1000);
    println!("1000 as a u16 is: {}", 1000 as u16);	

    // 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bit (MSB) get truncated.
	assert_eq!(1000 as u8, 232);
    println!("1000 as a u8 is : {}", 1000 as u8);
	
    // -1 + 256 = 255
	assert_eq!((-1i8) as u8, 255);
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // For positive numbers, this is the same as the modulus
	assert_eq!(1000 % 256, 232);
    println!("1000 mod 256 is : {}", 1000 % 256);

    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type. If the most significant
    // bit of that value is 1, then the value is negative.

    // Unless it already fits, of course.
	assert_eq!(128 as i16, 128);
    println!(" 128 as a i16 is: {}", 128 as i16);
    // 128 as u8 -> 128, whose two's complement in eight bits is:
	assert_eq!(128 as i8, -128);
    println!(" 128 as a i8 is : {}", 128 as i8);

    // repeating the example above
    // 1000 as u8 -> 232
	assert_eq!(1000 as u8, 232);
    println!("1000 as a u8 is : {}", 1000 as u8);
    // and the two's complement of 232 is -24
	assert_eq!(232 as i8, -24);
    println!(" 232 as a i8 is : {}", 232 as i8);
    
    // Since Rust 1.45, the `as` keyword performs a *saturating cast* 
    // when casting from float to int. If the floating point value exceeds 
    // the upper bound or is less than the lower bound, the returned value 
    // will be equal to the bound crossed.
    
    // 300.0 is 255
	assert_eq!(300.0_f32 as u8, 255);
    println!("300.0 is {}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
	assert_eq!(-100.0_f32 as u8, 0);
    println!("-100.0 as u8 is {}", -100.0_f32 as u8);
    // nan as u8 is 0
	assert_eq!(f32::NAN as u8, 0);
    println!("nan as u8 is {}", f32::NAN as u8);
    
    // This behavior incurs a small runtime cost and can be avoided 
    // with unsafe methods, however the results might overflow and 
    // return **unsound values**. Use these methods wisely:
    unsafe {
        // 300.0 is 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}

struct FieldStruct {
    first: u8,
    second: u16,
    third: u8,
	forth: String,
}

struct TupleStruct(u8, u16, u8, String);

struct UnitStruct;

enum WebEvent1 {
	PageLoad,
	PageUnload,
	Press(char),
	Paste(String), // max bytes 24
	Click { x: u32, y: u32, },	
}

enum WebEvent2 {
	PageLoad,
	PageUnload,
	Press(char),
	Paste(String),
	Click { x: u32, y: u32, },
	Draw { x: u16, y: u16, info: String, color: (u8, u8, u8, u8) }, // max bytes 32
}

fn literals() {
	println!("~~~~~~~literals~~~~~~~");
	let x = 1u8;
	let y = 2u32;
	let z = 3f32;
	
	let i = 1;
	let f = 1.0;
	
	//TODO: add new std::mem::size_of for new type
	println!("base type");
	println!("size of 'x as u8' in bytes: {}", std::mem::size_of_val(&x));
	println!("size of 'y as u32' in bytes: {}", std::mem::size_of_val(&y));
	println!("size of 'z as f32' in bytes: {}", std::mem::size_of_val(&z));
	println!("size of 'i default' in bytes: {}", std::mem::size_of_val(&i));
	println!("size of 'f default' in bytes: {}", std::mem::size_of_val(&f));
		
	println!("size of '()' in bytes: {}", std::mem::size_of::<()>());
	// tuple with only one element
	println!("size of '(u8,)' in bytes: {}", std::mem::size_of::<(u8,)>());
	println!("size of '&(u8,)' in bytes: {}", std::mem::size_of::<&(u8,)>());
	println!("size of '(u8, u8, u8)' in bytes: {}", std::mem::size_of::<(u8, u8, u8)>());
	println!("size of '&(u8, u8, u8)' in bytes: {}", std::mem::size_of::<&(u8, u8, u8)>());
	println!("size of '[i32; 0]' in bytes: {}", std::mem::size_of::<[i32; 0]>());
	println!("size of '[i32; 3]' in bytes: {}", std::mem::size_of::<[i32; 3]>());
	println!("size of '[FieldStruct; 3]' in bytes: {}", std::mem::size_of::<[FieldStruct; 3]>());
	println!("size of '&[FieldStruct; 3]' in bytes: {}", std::mem::size_of::<&[FieldStruct; 3]>());
	
	println!("custom type");
	//println!("size of 'str' in bytes: {}", std::mem::size_of::<str>());
	println!("size of '&str' in bytes: {}", std::mem::size_of::<&str>());
	println!("size of 'String' in bytes: {}", std::mem::size_of::<String>());
	println!("size of '&String' in bytes: {}", std::mem::size_of::<&String>());
	println!("size of 'FieldStruct' in bytes: {}", std::mem::size_of::<FieldStruct>());
	println!("size of '&FieldStruct' in bytes: {}", std::mem::size_of::<&FieldStruct>());
	println!("size of '&[i32]' in bytes: {}", std::mem::size_of::<&[i32]>());
	println!("size of '&[FieldStruct]' in bytes: {}", std::mem::size_of::<&[FieldStruct]>());
	println!("size of 'TupleStruct' in bytes: {}", std::mem::size_of::<TupleStruct>());
	println!("size of '&TupleStruct' in bytes: {}", std::mem::size_of::<&TupleStruct>());
	println!("size of 'UnitStruct' in bytes: {}", std::mem::size_of::<UnitStruct>());
	println!("size of '&UnitStruct' in bytes: {}", std::mem::size_of::<&UnitStruct>());	
	
	println!("size of 'WebEvent1' in bytes: {}", std::mem::size_of::<WebEvent1>());
	// error[E0573]: expected type, found variant `WebEvent::PageLoad`
	//println!("size of 'WebEvent::PageLoad' in bytes: {}", std::mem::size_of::<WebEvent::PageLoad>());
	println!("size of '&WebEvent1' in bytes: {}", std::mem::size_of::<&WebEvent1>());
	println!("size of 'WebEvent2' in bytes: {}", std::mem::size_of::<WebEvent2>());	
	println!("size of '&WebEvent2' in bytes: {}", std::mem::size_of::<&WebEvent2>());
	println!("size of 'Vec<i32>' in bytes: {}", std::mem::size_of::<Vec<i32>>());
	println!("size of '&Vec<i32>' in bytes: {}", std::mem::size_of::<&Vec<i32>>());
	
	println!("function type");
	//println!("size of 'Fn()' in bytes: {}", std::mem::size_of::<Fn()>());
}

type NanoSecond = u64;
type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64;

fn aliasing() {
	println!("~~~~~~~aliasing~~~~~~~");
	let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;
	
	println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}

fn main() {
	casting();
	literals();
	aliasing();
}