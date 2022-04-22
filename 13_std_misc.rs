fn print_knowledage_point(s: &str) {
	println!("~~~~~~~{}~~~~~~~", s);
}

use std::thread;

const NTHREADS: i32 = 3;

fn thread_example() {
	print_knowledage_point("thread example");
	
	let mut threads = vec![];
	
	// spawning native OS threads
	for i in 0..NTHREADS {
		threads.push(thread::spawn(move || {
			println!("this is thread: {}", i);
		}));
	}
	
	for child in threads {
		let _ = child.join();
	}
}

/*
The aliasing rules (one writable reference XOR many readable references) 
automatically prevent you from manipulating state that is visible to other threads. 
(Where synchronisation is needed, there are synchronisation primitives like Mutexes or Channels.)
*/
fn thread_map_reduce() {
	print_knowledage_point("thread map-reduce");
	
	let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

	let mut children = vec![];
	let chunked_data = data.split_whitespace();
	
	for (i, data_segment) in chunked_data.enumerate() {
		println!("data segment {} is \"{}\"", i, data_segment);
		
		children.push(thread::spawn(move || -> u32 {
			let result = data_segment
						 .chars()
						 .map(|c| c.to_digit(10).expect("should be a digit"))
						 .sum();
						 
			println!("data segment {}, result {}", i, result);
			
			result
		}));
	}
	
	// combine each thread's intermediate results into a single final sum.    
    // we use the "turbofish" ::<> to provide sum() with a type hint.
	let final_result = children.into_iter().map(|r| r.join().unwrap()).sum::<u32>();
	
	println!("Final sum result: {}", final_result);
}

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

fn thread_channel() {
	print_knowledage_point("thread channels");
	
	let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
	let mut children = Vec::new();
	
	for id in 0..NTHREADS {
		let thread_tx = tx.clone();
		
		let child = thread::spawn(move || {
			thread_tx.send(id).unwrap();
			
			println!("thread {} finished", id);
		});
		
		children.push(child);
	}
	
	let mut ids = Vec::with_capacity(NTHREADS as usize);
	for _ in 0..NTHREADS {
		ids.push(rx.recv());
	}
	
	for child in children {
		child.join().expect("oops! the child thread panicked");
	}
	
	println!("{:?}", ids);
}

use std::path::Path;

fn system_path() {
	print_knowledage_point("system path");
	
	let path = Path::new(".");
	
	println!("current path: {}", path.display());
	println!("current path: {:?}", path);
	
	let new_path = path.join("a").join("b");
	
	match new_path.to_str() {
		Some(s) => println!("new path is {}", s),
		None => println!("new path is not a valid UTF-8 sequence"),
	}
}

use std::fs::File;
use std::io::prelude::*;

fn system_file_open() {
	print_knowledage_point("system file open");
	
	let path = Path::new("10_macro_rules_TODO.rs");
	let display = path.display();
	
	let mut file = match File::open(&path) {
		Ok(file) => file,
		Err(why) => panic!("Couldn't open {} : {}", display, why),
	};
	
	let mut content = String::new();
	match file.read_to_string(&mut content) {
		Ok(size) => println!("{}, read {} bytes, contains:\n{}",
			display, size, content),
		Err(why) => panic!("Couldn't read {}: {}", display, why),
	}
}

static LOREM_IPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

fn system_file_create() {
	print_knowledage_point("system file create");
	
	let path = Path::new("lorem.txt");
	let display = path.display();
	
	let mut file = match File::create(&path) {
		Ok(file) => file,
		Err(why) => panic!("Couldn't create {} : {}", display, why),
	};
	
	match file.write_all(LOREM_IPSUM.as_bytes()) {
		Ok(_) => println!("{}, write all successed", display),
		Err(why) => panic!("Couldn't create {} : {}", display, why),
	}
}

use std::io::{self, BufRead};

fn read_lines<P>(filename: P) -> io::Result<io::Lines::<io::BufReader<File>>>
	where P: AsRef<Path>
{
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

fn system_file_readline() {
	print_knowledage_point("system file readline");
	
	if let Ok(lines) = read_lines("lorem.txt") {
		for line in lines {
			if let Ok(l) = line {
				println!("{}", l);
			}
		}
	}
}

use std::process::{Command, Stdio};
fn system_run_cmd() {
	print_knowledage_point("system run cmd");
	
	let output = Command::new("rustc")
		.arg("--version")
		.output().unwrap_or_else(|e| {
			panic!("Failed to execute cmd: {}", e);
		});
		
	if output.status.success() {
		let s = String::from_utf8_lossy(&output.stdout);
		
		print!("rustc succeeded and stdout was:\n{}", s);
	} else {
		let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc failed and stderr was:\n{}", s);
	}
}

static PANGRAM: &'static str =
"the quick brown fox jumped over the lazy dog\n";

fn system_run_cmd_with_input() {
	print_knowledage_point("system run cmd with input");
	
	let process = match Command::new("wc")
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn() {
		Err(why) => panic!("Couldn't spawn wc: {}", why),
		Ok(process) => process,
	};
		
	match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
		Err(why) => panic!("Couldn't write to wc stdin: {}", why),
		Ok(_) => println!("send pangram to wc"),
	}
	
	let mut s = String::new();
	match process.stdout.unwrap().read_to_string(&mut s) {
		Err(why) => panic!("Couldn't read from wc stdin: {}", why),
		Ok(_) => println!("wc response: \n{}", s),
	}
}

fn system_run_cmd_and_wait() {
	print_knowledage_point("system run cmd and wait");
	
	let mut child = Command::new("sleep").arg("1").spawn().unwrap();
	let _ = child.wait().unwrap();
	
	println!("reached end of main");
}

use std::fs::{self, OpenOptions};
use std::path::{PathBuf, MAIN_SEPARATOR};
//use std::os::windows;

// A simple implementation of `% cat path`
fn cat(path: &Path) -> io::Result<String> {
	let mut file = File::open(path)?;
	let mut s = String::new();
	/*
	match file.read_to_string(&mut s) {
		Ok(_) => Ok(s),
		Err(e) => Err(e),
	}
	or */
	file.read_to_string(&mut s)?
	Ok(s)
}

// A simple implementation of `% echo s > path`
fn echo(s: &str, path: &Path) -> io::Result<()> {
	let mut file = File::create(path)?;
	
	file.write_all(s.as_bytes())
}

// A simple implementation of `% touch path` (ignores existing files)
fn touch(path: &Path) -> io::Result<()> {
	match OpenOptions::new().create(true).write(true).open(path) {
		Ok(_) => Ok(()),
		Err(e) => Err(e),
	}
}

fn filesystem_operation() {
	print_knowledage_point("filesystem operation");
	
	println!("mkdir a");
	// Create a directory, returns `io::Result<()>`
	match fs::create_dir("a") {
		Err(why) => println!("! {:?}", why.kind()),
		Ok(_) => {},
	}
	
	println!("`echo hello > a/b.txt`");	
	let p1 = Path::new("a\\b.txt");
	echo("hello", &p1).unwrap_or_else(|why| {
		println!("! {:?}", why.kind());
	});
	assert!(p1.exists());
	
	println!("`mkdir -p a/c/d`");
	let p2 = Path::new("a\\c\\d");
    // Recursively create a directory, returns `io::Result<()>`
	fs::create_dir_all(&p2).unwrap_or_else(|why| {
		println!("! {:?}", why.kind());
	});
	assert!(p2.exists());
	
	println!("`touch a/c/e.txt`");
	let p3 = Path::new("a\\c\\e.txt");
	touch(&p3).unwrap_or_else(|why| {
		println!("! {:?}", why.kind());
	});
	assert!(p3.exists());
	
	println!("`ln -s ../b.txt a/c/b.txt`");
    // Create a symbolic link, returns `io::Result<()>`
	if cfg!(target_family = "windows") {
		println!("current target_family not support symlink");
	}
	/*{
		unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
			println!("! {:?}", why.kind());
		});
	} else {
		println!("current target_family is not unix");
	}*/
	
	
	println!("`cat a/b.txt`");
	match cat(&p1) {
		Ok(s) => println!("b contents: \n{}", s),
		Err(why) => println!("! {:?}", why.kind()),
	}
	
	println!("`ls a`");
    // Read the contents of a directory, returns `io::Result<Vec<Path>>`
	match fs::read_dir("a") {
		Err(why) => println!("! {:?}", why.kind()),
		Ok(paths) => {
			for path in paths {
				println!("> {:?}", path);
			}
		},
	}
	
	println!("`rm a/c/e.txt`");
    // Remove a file, returns `io::Result<()>`
	fs::remove_file(&p3).unwrap_or_else(|why| {
		println!("! {:?}", why.kind());
	});
	
	println!("`rmdir a/c/d`");
    // Remove an empty directory, returns `io::Result<()>`
	fs::remove_dir(&p2).unwrap_or_else(|why| {
		println!("! {:?}", why.kind());
	});	
}

fn main() {
	thread_example();
	thread_map_reduce();
	thread_channel();
	
	system_path();
	system_file_open();
	system_file_create();
	system_file_readline();
	
	system_run_cmd();
	system_run_cmd_with_input();
	//system_run_cmd_and_wait();
	
	filesystem_operation();	
}