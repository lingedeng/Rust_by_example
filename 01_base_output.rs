use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({}, {})", self.0, self.1)
	}
}

#[derive(Debug)]
struct Point2D {
	x: f64,
	y: f64,
}

impl fmt::Display for Point2D {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "x: {}, y: {}", self.x, self.y)
	}
}

#[derive(Debug)]
struct Complex {
	real: f64,
	imag: f64,
}

impl fmt::Display for Complex {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} + {}i", self.real, self.imag)
	}
}

struct List(Vec<i32>);

impl fmt::Display for List {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let vec = &self.0;
		
		// Try `write!` to see if it errors. If it errors, return
		// the error. Otherwise continue.
		write!(f, "[")?;
		
		for (count, v) in vec.iter().enumerate() {
			if count != 0 {
				write!(f, ", ")?;
			}
			write!(f, "{}: {}", count, v)?;
		}
		
		write!(f, "]")
	}
}

struct City {
	name: &'static str,
	lat: f32,
	lon: f32,
}

impl fmt::Display for City {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
		let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
		
		write!(f, "{}: {:.3}°{} {:.3}°{}", 
			self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
	}
}

#[derive(Debug)]
struct Color {
	red: u8,
	green: u8,
	blue: u8,
}

impl fmt::Display for Color {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		/*
		write!(f, "RGB ({}, {}, {}) 0x{:02X}{:02X}{:02X}",
			self.red, self.green, self.blue,
			self.red, self.green, self.blue)
		*/
		write!(f, "RGB ({red}, {green}, {blue}) 0x{red:02X}{green:0>2X}{blue:02X}",
			red = self.red, green = self.green, blue = self.blue)
	}
}

fn main() {
	let minmax = MinMax(0, 14);
	println!("Compare structures:");
	println!("Display: {}", minmax);
	println!("Debug: {:?}", minmax);
	
	let big_range = MinMax(-300, 300);
	let small_range = MinMax(-3, 3);
	println!("The big range is {big} and the small range is {small}",
		big = big_range, small = small_range);
		
	let point = Point2D {
		x: 3.3,
		y: 7.2,
	};
	println!("Compare structures:");
	println!("Display: {}", point);
	println!("Debug: {:?}", point);
	
	let complex = Complex {
		real: 3.3,
		imag: 7.2,
	};
	println!("Compare structures:");
	println!("Display: {}", complex);
	println!("Debug: {:?}", complex);
	
	let list = List(vec![1, 2, 3]);
	println!("{}", list);
	
	for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
	
	for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        //println!("{:?}", *color);
		println!("{}", color);
    }
}