/*

These are my notes on Chapter 3 of The Book.

Common Programming Concepts:

	First thing's first: All variables are immutable by default. We saw this
	last chapter but there's a lot to talk about here. Associated code is in
	the variables/ project. "Immutable" means once a value is assigned to a
	variable, that variable cannot be changed. This is the same as const in c.

	The book provides us with an example of something that won't work:*/

	fn main() {
		let x = 5;  //x assigned as immutable
		println!("The value of x is: {x}");
		x = 6;      //attempted reassignment of immutable variable: Illegal!
		println!("The value of x is: {x}");
	}

	/*

	Here is the exact error produced by the compiler:

	error[E0384]: cannot assign twice to immutable variable `x`
	 --> src/main.rs:4:5
	  |
	2 |     let x = 5;
	  |         -
	  |         |
	  |         first assignment to `x`
	  |         help: consider making this binding mutable: `mut x`
	3 |     println!("The value of x is: {x}");
	4 |     x = 6;
	  |     ^^^^^ cannot assign twice to immutable variable

	For more information about this error, try `rustc --explain E0384`.
	error: could not compile `variables` due to previous error

	The book talks about how to read this message, but I'd like to point out a
	few things about this message that I notice, too. First off, the compiler
	suggests making the variable mutable. While that would work here, I've
	already encountered many scenarios where changing mutability does not help
	but is the only suggestion offered by the compiler. So, the compiler, while
	helpful, is not magic and cannot produce solutions for you. Second, if the
	compiler suggestions cannot always be helpful, it's important to consider
	how we use this information. 

	It won't compile: does this mean our syntax is wrong somewhere, does it
	mean we have some logical inconsistency between lines, or does it mean we 
	need to reconsider at a larger scale how we are approaching some problem? 
	As I gain more experience, I feel like it is often the second issue 
	(logical inconsistency between lines), and then if trying to tackle the 
	second issue doesn't work, then it's a syntax thing. But, when the compiler 
	errors are not directly descriptive of the problem, or changing something 
	in a way suggested by the compiler does not move us closer to solving the 
	problem, then it's time to recognize we are doing something wrong, take a 
	step back, and reformulate our solution. I've wasted a lot of time chasing
	down errors that I believe exist when the process was initially flawed.

	Back to the book: Why are all values immutable by default? After all, if I,
	the programmer, am changing a value, surely I know why and need to do so?
	"Not so!" says Rust. Programs are complex beasts of massive amounts of tiny
	moving parts. If one piece relies on another to stay the same, while yet a
	third piece relies on it to change, our program will fail and, in the
	forest of machinery, become extremely difficult to reason about and fix. So
	Rust encourages us at a structural level to use constants wherever possible
	and leave mutability for when it is necessary. When errors happen, they are
	much easier to fix. Conversely, when making a variable mutable, it's clear
	to the programmer (and others!) that this data will change, so if errors
	are happening maybe start from here.

	We can fix the previous code by making x mut, as the compiler suggests:*/

	fn main() {
		let mut x = 5;  //x assigned as mutable
		println!("The value of x is {x}");
		x = 6;          //attempted reassignment of mutable variable: Legal!
		println!("The value of x is {x}");
	}

	/*

	Now, while all variables in Rust are immutable, we do also have explicit
	const variables. The keyword const in Rust means this value can never be
	mutable, and must have a type declared at assignment. ex:*/

	const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

	/*

	So, we see that 1) const variables are UPPERCASE_WITH_UNDERSCORES by naming
	convention, and that 2) it is good to show a little bit of where these very
	important values come from. Here it's the fact that 3 hours in seconds is
	60 minutes * 60 seconds * 3 hours, instead of just writing = 10800. Chapter
	17 has more information on what can and cannot be evaluated as const at
	compile time.

Shadowing:

	In the previous chapter, we wrote let guess = String::new(); followed by 
	guess = guess.trim().parse().expect("Nonnumeric character encountered!");.
	But in the previous section, we found that trying to reassign x throws an
	error. What gives?

	This is a feature called shadowing. If you have a variable foo and declare
	a new variable named foo in a scope that foo already exists, the new foo is
	the one which will be invoked by using that name, either until it is also
	shadowed or the scope ends. This really requires an example: */

	fn main() {
		let x = 5;

		let x = x + 1; //x now refers to this variable, whose value is 6.

		{
			let x = x * 2;  //x now refers to this variable, whose value is 12.
							//This happens because x in the outer scope refers
							//to the x of value 6, which overshadows the x with
							//value 5.
			println!("The value of x in the inner scope is: {x}"); //prints 12
		}

		println!("The value of x is: {x}"); //prints 6, as the value 5 x is
											//still shadowed by the value 6 x,
											//and the value 12 x left scope.
	}

	/*

	Doing things this way is not a violation of immutability. We are creating
	new variables with the same name, not changing our immutable variable. This
	might sound like a weird hack, but remember we are denoting these changes
	with the let keyword. This is also beneficial to us: it allows us to change
	the type associated with that name. Consider the following scenario: we 
	want to know how many spaces the user has entered:*/

	let spaces = "   ";
	let spaces = spaces.len();  //Shadowing turns our string into a u32. Also
								//note that we can keep some sort of logical
								//consistency within our name scheme without
								//introducing a ton of separate variables.

	let mut spaces = "   ";
	spaces = spaces.len(); //mutating a string to a u32 fails to compile

	/*

Data Types:

	Rust is a statically typed language with strong type inference. However,
	there are sometimes more than one type possible for inference, and we must
	specify what we want. We saw this in Chapter 2 in the line:*/

	let guess: u32 = "42".parse().expect("Not a number!");

	/*

	Because parse() can return several types (any numeric type), the compiler
	needs to be told what type we want it to be. Let's look at what types exist
	in Rust by default (i.e. without packages introducing their own).

	Scalar Types:

		These types represent individual values.

		Integer Types:

			Ints are numbers without fractions. We refer to them as signed or
			unsigned, and by their length. So in Rust, we can combine these
			designations to make a bunch of different types:

			length | signed | unsigned
			8-bit       i8      u8          //byte
			16-bit      i16     u16         //short
			32-bit      i32     u32         //int
			64-bit      i64     u64         //long
			128-bit     i128    u128        //double
			arch        isize   usize

			Signed numbers are stored in two's complement. Arch sizes depend
			on the architecture of the OS: They are 32 bits for 32-bit
			architecture, and 64 bits for 64-bit architectures.

			Numeric literals can use _ as a separator character, can use a type
			as a suffix, and a base as a prefix. E.g. 0xffu32 would give you
			the value 256 stored in a u32. 

			There are methods provided by Rust to handle overflows. 

			- wrapping_* functions wrap around to mins,
			- checked_* methods can return None on overflow,
			- overflowing_* methods return the overflow and a boolean,
			- saturating_* methods return the max/min value of the type instead
				of allowing the overflow.

		Floating Point Types:

			The only floating point types are f32 and f64, and function as you
			would expect. Specifically they conform to IEEE-754, in case you
			need to look that up to write an evil floating point bit hack or
			something.

		All numeric types support the basic operations you would expect between
		them:*/

		fn main() {
			// addition
			let sum = 5 + 10;

			// subtraction
			let difference = 95.5 - 4.3;

			// multiplication
			let product = 4 * 30;

			// division
			let quotient = 56.7 / 32.2;
			let truncated = -5 / 3; // Results in -1

			// remainder
			let remainder = 43 % 5;
		}

		/*

		Booleans:

			Booleans are one byte and store just a true or a false. If using a
			type annotation, that annotation is bool.

		Characters:

			char literals are specified with single quotes, and are four bytes
			in size. This is larger than a char literal in c, as Rust's is
			designed to represent the entire unicode space. As such, we can do
			things like:*/

			fn main() {
				let c = 'z';
				let z: char = 'ℤ'; // with explicit type annotation
				let heart_eyed_cat = '😻';
			}

			/*

			Which are a little funny.

Compound Types:

	Rust has two compound types by default: tuples and arrays.

	Tuples:

		Tuples are fixed-length lists of items which may or may not have the
		same type. They are declared as comma-separated lists in parentheses,
		and can have type annotations. An example:*/

		let tuple: (i32, f64, u8) = (500, 6.8, 1);

		//We can use pattern matching to extract the values:

		let (x, y, z) = tup; //x = 500, y = 6.8, z = 1

		println!("The value of z is: {z}");

		//or extract them by indexing with a period:

		{
			let x: (i32, f64, u8) = (500, 6.4, 1);
	
			let five_hundred = x.0;
	
			let six_point_four = x.1;
	
			let one = x.2;
		}

		/*

		Note that the tuple () is called the "unit" and is implicitly returned
		by any expression if it does not return any other value.

	Arrays:

		Arrays are like tuples, but every element must be of the same type. 
		They are useful when you want stack-allocated memory instead of heap
		-allocated memory for your collection (vectors use heap allocation).

		We can write arrays as lists within square brackets, and denote type:*/

		let a: [i32; 5] = [1, 2, 3, 4, 5];

		//We declare an array of 5 i32s and then fill it with i32s from
		//numeric literals. We also have some shorthand stuff:

		let a = [3; 5]; //creates an array [3, 3, 3]

		/*

		We can access elements of arrays by indexing as in c or similar.

		Accessing locations out of bounds of an array panics your program.

Functions:

	It's a programming language, we have functions, declare them with fn. Rust
	is very particular about naming convention; variables and functions are
	snake_case_like_this. Rust doesn't care where functions are defined, only
	that they are in a scope that can be seen by the caller. 

	Parameters:

		Parameters are declared in function definitions as fn foo(bar: type).
		Type declarations are mandatory.

	Statements and Expressions:

		Functions typically have bodies. These bodies are made of statements
		and expressions. Let's pull up the formal definitions:

			Statement: an instruction that performs an action but does not
			return a value. e.g. let y = 6; is a statement.

			Expressions: an instruction that evaluates to a resultant value.
			e.g. let x = y + 5; is an expression.

		Rust lets us combine statements and expressions in funny ways. Let's
		see an example:*/

		fn main() {
			let y = {
				let x = 3;
				x + 1
			};

			println("The value of y is: {y}"); //prints 4
		}

		/*

		Above, the block of code in {} is an expression, which is internally
		made up of a statement and an expression.

	Return Values:

		A big part of the usefulness of functions is being able to do some work
		and then return a value as the product of that work. When we declare
		return values in rust, we simply add an arrow (->) and the return type
		to the function signature. So, a function like this is perfectly valid
		Rust:*/

		fn five() -> i32 {
			5
		}

		/*

		It looks really funny but this function does in fact return 5 as i32.
		This is mostly pretty standard stuff, just syntactically different than
		some other languages.

Comments:

	//comments can be declared like this
	/* or in blocks like this */
	///and these are documentation comments!

Control Flow:

	If every program just ran the same set of instructions every time, they
	wouldn't be very useful now, would they? So we need to be able to execute
	code conditionally. Those are control structures.

if Expressions:

	Code locked behind an if statement only executes if the input to the if
	statement makes its condition evaluate true. We can also use the keyword
	else to declare blocks of code that should run in the case of a false.

	Note that rust conditionals only accept expressions evaluating as boolean;
	statements like "if number {" will fail.

	Like other languages we can use else if to define a case structure. It will
	stop evaluating the conditions once one condition in the else-if chain
	evaluates true.

Using if In let Statements:

	if is an expression! That means it evaluates to something and the something
	can be assigned somewhere. Here's an example:*/

	fn main() {
		let condition = true;
		let x = if condition {5} else {6};
	}

	/*

	There are some limitations here, and room for complexity. First off, we can
	have very lengthy blocks in let-ifs. That's the complexity part. Second,
	the blocks must all have the same return type. Because the compiler tries
	to infer the type of the variable being bound, having multiple possible
	return types would make this impossible.

Repetition With Loops:

	Rust provides several control statements to run code: loop, while, and for.

	loop:

		loop runs everything in its block indefinitely. It is equivalent to
		writing while(1) in c. To exit a structure involving loop, we need to
		use break. We could also use continue to restart the loop from the next
		iteration.

		break also functions as a return statement for loops. An expression
		provided on the same line as the break statement will be returned to
		the outside of the loop. An example:*/

		fn main() {
			let mut counter = 0;

			let result = loop {
				counter += 1;

				if counter == 10 {
					break counter * 2; //returns 20 to the loop exterior.
				}
			}
		}

		/*

		breaks and continues work on the level of the innermost loop. If we
		want to reference an outer loop with break or continue from an inner
		loop, we have to use loop labels. This is kind of a similar thing to
		labels in zachtronics games; loop labels always start with a '. ex:*/

		fn main() {
			let mut count = 0;

			'counting_up: loop {
				println!("count = {count}");
				let mut remaining = 10;

				loop {
					println!("remaining = {remaining}");
					if remaining == 9 {
						break; //exits inner loop
					}
					if count == 2 {
						break 'counting_up; //exits outer loop
					}
					remaining -= 1;
				}

				count += 1;
			}
			println!("End count = {count}");
		}

		/*

		In the above code, we enter 'counting_up, remaining = 10, enter inner
		loop. In the inner loop, remaining goes to 9, we loop, and then the
		inner loop breaks. In the outer loop, we hit count += 1 for a count of
		1. We go back to the start of 'counting_up, repeat this process again
		so that count = 2, and then enter the inner loop again. Now, remaining
		= 10, but count = 2, so we encounter the statement break 'counting_up;
		and the outer loop is broken. We encounter the print statement with
		count = 2.

		This behavior is weird and not familiar to me. Neat.

	while:

		While loops work exactly as you would expect from c.

	for:

		for works the way you would expect it to from Python. If we want to use
		a range of numbers, the syntax is (x..y), which gives a range of vals
		in interval [x, y). We can also reverse the range using .rev(). There
		is documentation for ranges in Range from the standard library for
		further reading.

Conclusion:

	That's the chapter. There's a lot here, and The Book suggests some of the
	following exercises:

	- Convert temperatures between Fahrenheit and Celsius.
	- Generate the nth Fibonacci number.
	- Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” 
	taking advantage of the repetition in the song.
