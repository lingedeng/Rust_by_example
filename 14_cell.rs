use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell};

fn print_knowledage_point(s: &str) {
	println!("~~~~~~~{}~~~~~~~", s);
}

fn no_inside_mutability() {
	print_knowledage_point("no inside mutability");
	
	let shared_maps: Rc<HashMap<&str, i32>> = Rc::new(HashMap::new());
	
	// error[E0596]: cannot borrow data in an `Rc` as mutable
	// trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Rc<HashMap<&str, i32>>`
	//shared_maps.insert("whoami", 3);
	//shared_maps.insert("bob", 7);
	//shared_maps.insert("jack", 10);
	
	let totals: i32 = shared_maps.values().sum();
	println!("{}", totals);
}

fn inside_mutability() {
	print_knowledage_point("inside mutability");
	
	let shared_maps = Rc::new(RefCell::new(HashMap::new()));
	
	// panicked without a new block
	// panicked at 'already mutably borrowed: BorrowError'
	//{
	{
		let mut tmp_mut_maps = shared_maps.borrow_mut();
		tmp_mut_maps.insert("whoami", 3);
		tmp_mut_maps.insert("bob", 7);
		tmp_mut_maps.insert("jack", 10);
	}
	//}
	
	let totals: i32 = shared_maps.borrow().values().sum();
	println!("{}", totals);
}

mod post {
	// without following 'use', error[E0412]: cannot find type `RefCell` in this scope
	use std::cell::{RefCell, Ref};
	
	pub struct Post {
		content: String,
	}

	impl Post {
		pub fn new() -> Post {
			Post { content: String::new() }
		}
		
		pub fn draft(&mut self) {
			self.content.push_str("Try write something...");
		}
		
		pub fn content(&self) -> &str {
			&self.content
		}
	}
	
	pub struct Post2 {
		content: RefCell<String>,
	}
	
	impl Post2 {
		pub fn new() -> Post2 {
			Post2 { content: RefCell::new(String::new()) }
		}
		
		pub fn draft(&self) {
			self.content.borrow_mut().push_str("Try write something...");
		}
		
		pub fn content(&self) -> Ref<'_, String> {
			// error[E0515]: cannot return value referencing temporary value
			//&self.content.borrow()
			// error[E0507]: cannot move out of dereference of `Ref<'_, String>`
			//*self.content.borrow()
			self.content.borrow()
		}
	}
}

fn no_logically_immutable_methods() {
	print_knowledage_point("no logically immutable methods");
	
	// 随着代码的改变产生的编译错误也可能发生改变, 有些后面产生的错误会覆盖前面的错误
	// error[E0451]: field `content` of struct `Post` is private
	//let post1 = post::Post { content: String::new() };
	// error[E0616]: field `content` of struct `Post` is private
	//println!("post1 content: {}", post1.content);
	//println!("post1 content: {}", post1.content());
	
	let mut post = post::Post::new();
	post.draft();
	println!("draft post content: {}", post.content());
}

fn logically_immutable_methods() {
	print_knowledage_point("logically immutable methods");

	let post = post::Post2::new();
	post.draft();
	println!("draft post content: {}", post.content());
}

fn main() {
	no_inside_mutability();
	inside_mutability();
	
	no_logically_immutable_methods();
	logically_immutable_methods();
}