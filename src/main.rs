use const_format::formatcp;

#[path = "./db/_db.rs"] mod db;


fn main() {
    let curry = |x| move |y| x + y;
	let add_to_one = curry(1);
	let add_two = add_to_one(2);
	println!("{}", add_two);
	let subj = "me";
	let s = formatcp!(db::dbQueries::test, subj);
	let f = subj;
	println!("{}",s);
}