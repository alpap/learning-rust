fn main(){
	ex_1();
	ex_2();
}

fn ex_1(){
	println!("My name is {0}, {1} {0}", "Bond", "James");


	#[allow(dead_code)]
    struct Structure(i32);
	// However, custom types such as this structure require more complicated
    // handling. This will not work.
    // println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.
}


fn ex_2(){
	let pi = 3.141592;
	let formated_num = format!("{:.2}",pi);
	println!("{}",formated_num);
	assert_eq!("3.14", formated_num);
	println!("{}",format!("{:.*}",2,pi));
}