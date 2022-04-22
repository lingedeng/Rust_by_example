use std::pin::Pin;
use std::mem;
use std::marker::PhantomPinned;
use std::ptr::NonNull;

// For T: Unpin, Pin<Box<T>> and Box<T> function identically, as do Pin<&mut T> and &mut T.
// Note that pinning and Unpin only affect the pointed-to type P::Target, not the pointer type P itself that got wrapped in Pin<P>.
// pinning和Unpin只影响借用或智能指针指向的类型，而非由Pin<P>封装的借用或智能指针类型本身
fn pin1() {
	println!("~pin1~");
	
	let mut x = 5;
	let mut y = 7;
	
	println!("x: {}, y: {}", x, y);
	// 生成一个临时变量，类型为&mut i32，传递给Pin::new
	let x_pin: Pin<&mut i32> = Pin::new(&mut x);
	let y_pin = Pin::new(&mut y);
	
	mem::swap(x_pin.get_mut(), y_pin.get_mut());
	// x_pin & y_pin was consume by get_mut()
	//println!("x1_pin: {}, y1_pin: {}", x1_pin, y1_pin);
	println!("x: {}, y: {}", x, y);
}

fn pin2() {
	println!("~pin2~");
	// 左值中附加的关键字用于添加到类型上，而右值中附加的关键字用于生成类型
	let x: &mut i32 = &mut 5;
	let ref mut y: i32 = 7;
	
	println!("x: {}, y: {}", x, y);
	
	// &mut T不支持Copy特性，&T支持Copy特性	
	let x_pin = Pin::new(x);	
	let y_pin: Pin<&mut i32> = Pin::new(y);
	
	mem::swap(x_pin.get_mut(), y_pin.get_mut());
	// error[E0382]: borrow of moved value: `x_pin`
	//println!("x: {}, y: {}", x_pin, y_pin);
	// error[E0382]: borrow of moved value: `x`
	//println!("x: {}, y: {}", x, y);
	// y can access because mutable references are automatically reborrowed instead of being moved 
	// when the target type is known to be a mutable reference without the compiler having to do any type inference.
	println!("y: {}", y);
}

fn pin3() {
	println!("~pin3~");
	
	let x = Box::new(5);
	let y = Box::new(7);
	
	println!("x: {}, y: {}", x, y);
	
	// move Box<i32> into Pin
	let mut x_pin = Pin::new(x);
	let mut y_pin = Pin::new(y);
	
	// error[E0599]: no method named `get_mut` found for struct `Pin<Box<{integer}>>` in the current scope
	//mem::swap(x_pin.get_mut(), y_pin.get_mut());
	mem::swap(x_pin.as_mut().get_mut(), y_pin.as_mut().get_mut());
	println!("x: {}, y: {}", x_pin, y_pin);
	//println!("x: {}, y: {}", x, y);
}

struct Unmovable {
	data: String,
	slice: NonNull<String>,
	_pin: PhantomPinned,
}

impl Unmovable {
	fn new(data: String) -> Pin<Box<Self>> {
		let res = Unmovable {
			data,
			slice: NonNull::dangling(),
			_pin: PhantomPinned,
		};
		
		let mut boxed = Box::pin(res);
		
		let slice = NonNull::from(&boxed.data);
				
		unsafe {
			let mut_ref: Pin<&mut Self> = boxed.as_mut(); //Pin::as_mut(&mut boxed);
			Pin::get_unchecked_mut(mut_ref).slice = slice;
		}		
		// error[E0277]: `PhantomPinned` cannot be unpinned
		// within `Unmovable`, the trait `Unpin` is not implemented for `PhantomPinned`
		//boxed.as_mut().get_mut().slice = slice;		
		
		boxed
	}
}

fn pin4() {
	println!("~pin4~");
	
	let unmoved = Unmovable::new(String::from("hello"));
	
	let mut still_unmoved = unmoved;
	
	assert_eq!(still_unmoved.slice, NonNull::from(&still_unmoved.data));
	
	// Since our type doesn't implement Unpin, this will fail to compile:
	//let mut new_unmoved = Unmovable::new("world".to_string());
	//mem::swap(&mut *still_unmoved, &mut *new_unmoved);
}


// Pin<P> prevents certain values (pointed to by pointers wrapped in Pin<P>) from being moved by 
// making it impossible to call methods that require &mut T on them (like mem::swap).
fn main() {
	pin1();
	pin2();
	pin3();
	pin4();
}