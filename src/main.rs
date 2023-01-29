use text_io::read;
fn main() {
	// let nyan = 10;
	// println!("hewo world! ^w^");
	// println!("uwu let's try a new line~");
	// println!("time to run code i dont understand");
	// println!("{}",nyan);
	// let nyan = nyan + 15;
	// println!("{}",nyan);
	// println!("{}",nyan);
	// println!("im tired");
	// let nyan: i32 = 11 + 12;
	// println!("{}",nyan);
	println!("input desired operation (+ - * /)");
	let get: String = read!("{}\n"); // i am sorry for this mess
	if get == "+" {
		println!("input first var");
		let one: f32 = read!("{}\n");
		println!("input second var");
		let two: f32 = read!("{}\n");
		print!("{}",one);
		print!(" + ");
		print!("{}",two);
		print!(" = ");
		print!("{}",one + two);
		println!(); // it likes to glitch out without this? why?	
	} else {	
		if get == "-" {
		println!("input first var");
		let one: f32 = read!("{}\n");
		println!("input second var");
		let two: f32 = read!("{}\n");
		print!("{}",one);
		print!(" - ");
		print!("{}",two);
		print!(" = ");
		print!("{}",one - two);
		println!(); 
	} else {
		if get == "*" {
		println!("input first var");
		let one: f32 = read!("{}\n");
		println!("input second var");
		let two: f32 = read!("{}\n");
		print!("{}",one);
		print!(" * ");
		print!("{}",two);
		print!(" = ");
		print!("{}",one * two);
		println!(); 
	} else {
		if get == "/" {
		println!("input first var");
		let one: f32 = read!("{}\n");
		println!("input second var");
		let two: f32 = read!("{}\n");
		print!("{}",one);
		print!(" / ");
		print!("{}",two);
		print!(" = ");
		print!("{}",one / two);
		println!(); 
	} else {
		println!("Error - have you entered a valid operation?")
	}}}}	
}