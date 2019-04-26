use std::fmt;

struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct DebugPrintable(i32);

fn main(){
	strct();
	db();
	disp();
}

fn strct(){
	#[derive(Debug)]
	struct Structure(i32);

	// Put a `Structure` inside of the structure `Deep`. Make it printable
	// also.
	#[derive(Debug)]
	struct Deep(Structure);
	// Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));
}


fn db(){
	#[derive(Debug)]
	struct Person<'a> {
		name: &'a str,
		age: u8
	}

	let name = "Peter";
	let age = 27;
	let peter = Person { name, age };

	// Pretty print
	println!("{:#?}", peter);


}



#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// Use `self.number` to refer to each positional data point.
		write!(f, "({}, {})", self.0, self.1)
	}
}

fn disp(){

	let min=MinMax(1,1);

	println!("{}",min)
}