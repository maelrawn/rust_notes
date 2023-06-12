// Exercises suggested at the end of the third chapter of The Book:
// - Convert temperatures between Fahrenheit and Celsius.
// - Generate the nth Fibonacci number.
// - Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” 
// taking advantage of the repetition in the song.

fn celsius_to_fahrenheit(celsius: i32) -> i32 {
	(celsius * 9/5) + 32
}

fn fahrenheit_to_celsius(fahrenheit: i32) -> i32 {
	(fahrenheit - 32) * 5/9
}

fn nth_fib(n: u32) -> u32 {
	let mut f1 = 0;
	let mut f2 = 1;
	let mut count = 2;
	while count < n {
		f2 += f1;
		f1 = f2 - f1;
		count += 1;
	}
	f2
}

fn christmas_carol() {
	let carol_items: [&str; 12] = [
		"A partridge in a pear tree",
		"Two turtle doves",
		"Three french hens",
		"Four calling birds",
		"Five golden rings",
		"Six geese a-laying",
		"Seven swans a-swimming",
		"Eight maids a-milking",
		"Nine ladies dancing",
		"Ten lords a-leaping",
		"Eleven pipers piping",
		"Twelve drummers drumming"
	];
	for n in 1..12 {
		println!("On the {n}th day of Christmas, my true love gave to me");

		for item in carol_items[0..=n].iter().rev() {
			println!("{:?}", item);
		}
	}
}

fn main() {
	let celsius = 44;
	let fahrenheit = 91;
	let n = 10;
	println!("{celsius}c to f: {:?}", celsius_to_fahrenheit(celsius));
	println!("{fahrenheit}f to c: {:?}", fahrenheit_to_celsius(fahrenheit));
	println!("The {n}th fibonacci number is: {:?}", nth_fib(n));
	christmas_carol();
}