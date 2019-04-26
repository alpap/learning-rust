use std::fmt;

#[derive(Debug)]
struct Complex  {
    real: f64,
    imaginary: f64,
}

impl fmt::Display for Complex  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "Complex {{ real: {}, imaginary: {} }}", self.real, self.imaginary)
    }
}

fn main(){
	let num=Complex {real:3.3,imaginary:7.2};
	println!("{}",num)
}