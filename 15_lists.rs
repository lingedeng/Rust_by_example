use std::mem;

/*
enum Link {
	Empty,
	More(Box<Node>),
}
*/
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
	elem: T,
	next: Link<T>,
}

struct List<T> {
	head: Link<T>,
}

impl<T> List<T> {
	fn new() -> Self {
		List {
			head: None,
		}
	}
	
	// 新结点的next结点为List.head指向的结点
	fn push(&mut self, elem: T) {
		let new_node = Box::new(Node {
			elem: elem,
			// error[E0507]: cannot move out of `self.head` which is behind a mutable reference
			//next: self.head,
			next: self.head.take(),
		});
		
		self.head = Some(new_node);
	}
	
		
	fn pop(&mut self) -> Option<T> {
		/*
		// error[E0507]: cannot move out of `self.head.0` which is behind a mutable reference
		match self.head {
			Link::Empty => None,			
			Link::More(boxed_node) => {
				self.head = boxed_node.next;
				Some(boxed_node.elem)
			}
		}
		*/
		/*
		// error[E0506]: cannot assign to `self.head` because it is borrowed
		// error[E0507]: cannot move out of `boxed_node.next` which is behind a shared reference
		match &self.head {
			Link::Empty => None,
			Link::More(boxed_node) => {
				self.head = boxed_node.next;
				Some(boxed_node.elem)
			}
		}
		*/
		/*
		match self.head.take() {
			None => None,
			Some(boxed_node) => {
				self.head = boxed_node.next;
				Some(boxed_node.elem)
			}
		}
		*/
		self.head.take().map(|boxed_node| {
			self.head = boxed_node.next;
			boxed_node.elem
		})
	}

	fn peek(&self) -> Option<&T> {
		// Converts from &Option<T> to Option<&T>
		self.head.as_ref().map(|boxed_node_ref| {
			&boxed_node_ref.elem
		})
	}
	
	fn peek_mut(&mut self) -> Option<&mut T> {
		// Converts from &mut Option<T> to Option<&mut T>
		self.head.as_mut().map(|boxed_node_ref| {
			&mut boxed_node_ref.elem
		})
	}
}

impl<T> Drop for List<T> {
	fn drop(&mut self) {
		let mut cur_link = self.head.take();
		while let Some(boxed_node) = cur_link {
			cur_link = boxed_node.next;
		}
	}
}

struct IntoIter<T>(List<T>);

impl<T> List<T> {
	fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}
}

impl<T> Iterator for IntoIter<T> {
	type Item = T;
	
	fn next(&mut self) -> Option<Self::Item> {
		self.0.pop()
	}
}

struct Iter<'a, T> {
	next: Option<&'a Node<T>>
}

impl<T> List<T> {
	fn iter(&self) -> Iter<T> {
		Iter {
			// error[E0515]: cannot return reference to local data `*node`
			// error[E0507]: cannot move out of `self.head` which is behind a shared reference
			// node: Box<Node<T>>
			//next: self.head.map(|node| &*node),
			// node: &Box<Node<T>>			
			//next: self.head.as_ref().map(|node| &**node),
			// Converts from Option<T> (or &Option<T>) to Option<&T::Target>
			// return: Option<&Node<T>>
			next: self.head.as_deref(),
		}
	}
}

impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;
	
	fn next(&mut self) -> Option<Self::Item> {
		self.next.map(|node| {
			self.next = node.next.as_ref().map(|node| &**node);
			&node.elem
		})
	}
}

fn test_basics() {
	let mut list = List::new();
	
	assert_eq!(list.pop(), None);
	
	list.push(1);
	list.push(2);
	list.push(3);
	
	assert_eq!(list.pop(), Some(3));
	assert_eq!(list.pop(), Some(2));
	
	list.push(4);
	list.push(5);
	assert_eq!(list.pop(), Some(5));
	assert_eq!(list.pop(), Some(4));
	
	assert_eq!(list.pop(), Some(1));
	assert_eq!(list.pop(), None);
}

fn test_peek() {
	let mut list = List::new();
	list.push("who");
	list.push("am");
	list.push("i");
	
	assert_eq!(list.peek(), Some(&"i"));
	
	assert_eq!(list.pop(), Some("i"));
	assert_eq!(list.pop(), Some("am"));
	assert_eq!(list.pop(), Some("who"));
	
	list.push("whoami");
	list.peek_mut().map(|v| {
		*v = "iamwho";
	});
	assert_eq!(list.pop(), Some("iamwho"));
}

fn test_into_iter() {
	let mut list = List::new();
	list.push(1); list.push(2); list.push(3);
	
	let mut iter = list.into_iter();
	assert_eq!(iter.next(), Some(3));
	assert_eq!(iter.next(), Some(2));
	assert_eq!(iter.next(), Some(1));
}

fn test_iter() {
	let mut list = List::new();
	list.push(1);
	list.push(2);
	list.push(3);

	let mut iter = list.iter();
	assert_eq!(iter.next(), Some(&3));
	assert_eq!(iter.next(), Some(&2));
	assert_eq!(iter.next(), Some(&1));
	assert_eq!(iter.next(), None);
}

fn main() {
	test_basics();
	test_peek();
	test_into_iter();
	test_iter();
}