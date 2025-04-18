fn main() {
    let curry = |x| move |y| x + y;
	let add_to_one = curry(1);
	let add_two = add_to_one(2);
	println!("{}", add_two);
}