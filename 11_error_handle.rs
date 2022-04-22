fn print_knowledage_point(s: &str) {
	println!("~~~~~~~{}~~~~~~~", s);
}

fn drink(beverage: &str) {
	if beverage == "lemonade" { panic!("AAAaaaaa!!!!"); }
	
	println!("Some refreshing {} is all I need.", beverage);
}

fn error_panic() {
	print_knowledage_point("error panic");
	
	drink("water");
	//drink("lemonade");
}

/*
These cases can either be explicitly handled via match or implicitly with unwrap. 
Implicit handling will either return the inner element or panic.
it's possible to manually customize panic with expect, but unwrap otherwise leaves us with a less meaningful output than explicit handling.
*/
fn give_adult(drink: Option<&str>) {
	match drink {
		Some("lemonade") => println!("Yuck! Too sugary."),
		Some(inner) => println!("{}? How nice.", inner),
		None => println!("No drink? Oh well"),
	}
}

fn give_child(drink: Option<&str>) {
	//let inside = drink.unwrap();
	let inside = drink.expect("Nothing drink for me!!!");
	if inside == "lemonade" { panic!("AAAaaaaa!!!!"); }
	
	println!("I love {}s!!!!", inside);
}

fn error_option() {
	print_knowledage_point("error Option");
	
	let water  = Some("water");
    let lemonade = Some("lemonade");
    let void  = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing: Option<&str> = None;
	
	give_child(coffee);
	//give_child(nothing);
}

/*
If x is an Option, then evaluating x? will return the underlying value if x is Some, 
otherwise it will terminate whatever function is being executed and return None.
*/
struct Person {
	job: Option<Job>,
}

struct Job {
	phone_number: Option<PhoneNumber>,
}

struct PhoneNumber {
	area_code: Option<u8>,
	number: u32,
}

impl Person {
	fn work_phone_area_code(&self) -> Option<u8> {
		self.job.as_ref()?.phone_number.as_ref()?.area_code
	}
}

fn option_unpacking_with_question_mark() {
	print_knowledage_point("unpacking options with ?");
	let person = Person {
		job: Some(Job {
			phone_number: Some(PhoneNumber {
				area_code: Some(25),
				number: 439222222,
			})
		})
	};
	
	println!("person's phone area code: {:?}", person.work_phone_area_code()); 
}

use std::num::ParseIntError;

type AliasedError<T> = Result<T, ParseIntError>;

fn multiply(first_num_str: &str, second_num_str: &str) -> AliasedError<i32> /*Result<i32, ParseIntError>*/ {
	match first_num_str.parse::<i32>() {
		Ok(first_num) => {
			match second_num_str.parse::<i32>() {
				Ok(second_num) => Ok(first_num * second_num),
				Err(e) => Err(e),
			}
		}
		Err(e) => Err(e),
	}
}

fn multiply_combinator(first_num_str: &str, second_num_str: &str) -> AliasedError<i32> /*Result<i32, ParseIntError>*/ {
	first_num_str.parse::<i32>().and_then(|first_num| {
		second_num_str.parse::<i32>().map(|second_num| first_num * second_num)
	})
}

fn multiply_early_return(first_num_str: &str, second_num_str: &str) -> AliasedError<i32> /*Result<i32, ParseIntError>*/ {
	//let mut first_num = 0;
	//let mut second_num = 0;
	
	let first_num = match first_num_str.parse::<i32>() {
		Ok(v) => v,
		Err(e) => return Err(e),
	};
	
	let second_num = match second_num_str.parse::<i32>() {
		Ok(v) => v,
		Err(e) => return Err(e),
	};
	
	Ok(first_num * second_num)
}

fn multiply_question_mark(first_num_str: &str, second_num_str: &str) -> AliasedError<i32> {
	let first_num = first_num_str.parse::<i32>()?;
	let second_num = second_num_str.parse::<i32>()?;
	
	Ok(first_num * second_num)
}

fn print_result_ref(result: &AliasedError<i32>/*Result<i32, ParseIntError>*/) {
	match result {
		// num: &i32, e: &ParseIntError
		// match时使用result或&result，对num和e的类型没有影响
		Ok(num) => println!("{}", num),
		Err(e) => println!("{}", e),
	}
}

fn print_result(result: AliasedError<i32>/*Result<i32, ParseIntError>*/) {
	match result {
		// num: i32, e: ParseIntError
		Ok(num) => println!("{}", num),
		//Err(e) => println!("{}", e),
		Err(ref e) => println!("{}", e),
	}
	
	// when Err(e) => println!("{}", e),
	// report: error[E0382]: borrow of partially moved value: `result`
	println!("print_result: {:?}", result);
}

fn error_result() {
	print_knowledage_point("error and_then combinators & aliases & early return & question mark");
	
	let (n1_ok, n2_ok, n_err) = ("11", "3", "t");	
		
	let result = multiply(&n1_ok, &n2_ok);
	print_result_ref(&result);
	println!("result: {:?}", result);
		
	let result = multiply_early_return(&n_err, &n2_ok);
	print_result_ref(&result);
		
	let result = multiply_combinator(&n1_ok, &n2_ok);
	print_result(result);
	
	//println!("result: {:?}", result);
	
	let result = multiply_question_mark(&n1_ok, &n_err);
	print_result(result);	
}

fn double_first(vec: Vec<&str>) -> i32 {
	let first = vec.first().unwrap();
	2 * first.parse::<i32>().unwrap()
}

fn double_first2(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
	vec.first().map(|first| {
		first.parse::<i32>().map(|n| n * 2)
	})
}

fn double_first3(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
	let opt = vec.first().map(|first| {
		first.parse::<i32>().map(|n| n * 2)
	});
	
	opt.map_or(Ok(None), |r| r.map(Some))
}

fn error_diff_return() {
	print_knowledage_point("error diff return");
	
	let numbers = vec!["12", "22", "32"];
	let empty: Vec<&str> = vec![];
	let strings = vec!["tofu", "93", "18"];
	
	/*
	//println!("The first doubled is {}", double_first(numbers));
	
	// thread 'main' panicked at 'called `Option::unwrap()` on a `None` value'
	//println!("The first doubled is {}", double_first(empty));
	
	// thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }'
	//println!("The first doubled is {}", double_first(strings));
	
	println!("The first doubled is {:?}", double_first2(numbers));
	println!("The first doubled is {:?}", double_first2(empty));
	println!("The first doubled is {:?}", double_first2(strings));
	*/
	
	println!("The first doubled is {:?}", double_first3(numbers));
	println!("The first doubled is {:?}", double_first3(empty));
	println!("The first doubled is {:?}", double_first3(strings));
}

use std::fmt;

type DFResult<T> = std::result::Result<T, DoubleError>;

struct DoubleError(&'static str);

impl fmt::Display for DoubleError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

fn double_first_4(vec: Vec<&str>) -> DFResult<i32> {
	vec.first()
		// Transforms the Option<T> into a Result<T, E>
		.ok_or(DoubleError("vec is empty"))
		// Returns None if the option is None, otherwise calls f with the wrapped value and returns the result
		.and_then(|first| {
			first.parse::<i32>()
				// Maps a Result<T, E> to Result<T, F> by applying a function to a contained Err value, leaving an Ok value untouched
				.map_err(|_| DoubleError("parse first elem to i32 failed"))
				// Maps a Result<T, E> to Result<U, E> by applying a function to a contained Ok value, leaving an Err value untouched
				.map(|n| 2 * n)
		})
}

fn print_df(result: DFResult<i32>) {
	match result {
		Ok(v) => println!("The first doubled is {}", v),
		Err(err) => println!("Error: {}", err),
	}
}

fn error_own_type() {
	print_knowledage_point("error own type");
	
	let numbers = vec!["12", "22", "32"];
	let empty: Vec<&str> = vec![];
	let strings = vec!["tofu", "93", "18"];
	
	print_df(double_first_4(numbers));
	print_df(double_first_4(empty));
	print_df(double_first_4(strings));
}

use std::error;

type DynDfError<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "invaild first item to double")
	}
}

impl error::Error for EmptyVec {}

fn double_first_5(vec: Vec<&str>) -> DynDfError<i32> {
	/*
	vec.first()
		.ok_or(EmptyVec.into())
		.and_then(|first| {
			first.parse::<i32>()
				.map_err(|e| e.into())
				.map(|n| 2 * n)
		})
	*/
	
	// if you ? where the error is convertible to the return type, it will convert automatically.
	let first = vec.first().ok_or(EmptyVec)?;
	let parsed = first.parse::<i32>()?;
	Ok(2 * parsed)
}

fn print_dyndf(result: DynDfError<i32>) {
	match result {
		Ok(v) => println!("The first doubled is {}", v),
		Err(err) => println!("Error: {}", err),
	}
}

fn error_boxed_dyn_return() {
	print_knowledage_point("error boxed dyn return");
	
	let numbers = vec!["12", "22", "32"];
	let empty: Vec<&str> = vec![];
	let strings = vec!["tofu", "93", "18"];
	
	print_dyndf(double_first_5(numbers));
	print_dyndf(double_first_5(empty));
	print_dyndf(double_first_5(strings));
}

fn error_iterator() {
	print_knowledage_point("error iterator");
	
	println!("~iterating over Results~");
	let strings = vec!["93", "tofu", "18"];
	let numbers: Vec<_> = strings
		//.into_iter()
		.iter()
		.map(|s| s.parse::<i32>())
		.collect();
	println!("Result: {:?}", numbers);
	println!("strings: {:?}", strings);
	
	println!("~ignore the failed item~");
	let numbers: Vec<_> = strings
		.iter()
		.filter_map(|s| s.parse::<i32>().ok())
		.collect();
	println!("Result: {:?}", numbers);
	
	println!("~fail the entire operation when Result::Err is found~");
	// Result implements FromIter so that a vector of results (Vec<Result<T, E>>) can be turned into a result with a vector (Result<Vec<T>, E>). 
	// Once an Result::Err is found, the iteration will terminate. just return Result::Err
	let numbers: Result<Vec<_>, _> = strings
		.iter()
		.map(|s| s.parse::<i32>())
		.collect();
	println!("Result: {:?}", numbers);	
	
	println!("~separate collection~");
	let (numbers, errors): (Vec<_>, Vec<_>) = strings
		.iter()
		.map(|s| s.parse::<i32>())
		.partition(Result::is_ok);
	println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
	
	let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
	println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}

fn main() {
	error_panic();
	error_option();
	
	option_unpacking_with_question_mark();
	
	error_result();
	
	error_diff_return();
	error_own_type();
	error_boxed_dyn_return();
	
	error_iterator();
}

/*
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
	let num_str = "e";
	let num = match num_str.parse::<i32>() {
		Ok(number) => number,
		Err(e) => return Err(e),
	};
	
	println!("number: {}", num);
	Ok(())
}
*/