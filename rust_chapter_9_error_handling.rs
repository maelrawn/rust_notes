/*

These are my notes on Chapter 9 of The Book.

Rust is designed around acknowledging and handling errors. This makes us write
more code up front, but makes that code more robust by nature. 

The two error categories are recoverable and unrecoverable errors. Recoverable
errors allow us to attempt a fix, like asking for a different filepath to check
for something at. Unrecoverable errors are always because of bugs, and require
us to stop the program immediately.

Rust is in a minority of languages that distinguish between these types of
errors. We use the enum Result<T, E> for recoverables and panic! for 
unrecoverables.

Unrecoverable Errors with panic!:

	If there's nothing we can do about an error, we usually want to panic!.
	Situations where a panic occurs happen two ways: do something illegal that
	the compiler could not prevent you from doing (access past end of array),
	or explicitly call the panic! macro. Panics by default print a failure
	message, unwind, clean up the stack, and quit. We can have Rust display the
	call stack when a panic occurs for easier debugging.

	Unwinding is an expensive operation. If we don't want to do that, we can
	instead abort, which leaves the cleanup process to the operating system.
	Disabling this unwinding behavior ends up creating a smaller binary, and we
	can do so by setting panic = 'abort' under [profile] in Cargo.toml.

	We can call panic ourselves, like we noted above:*/

	fn main() {
		panic!("crash and burn");
	}

	/*

	Running this program will produce the following output:

	$ cargo run
	   Compiling panic v0.1.0 (file:///projects/panic)
	    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
	     Running `target/debug/panic`
	thread 'main' panicked at 'crash and burn', src/main.rs:2:5
	note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

	The call to panic! is what produces the last two lines here. The first line
	shows which thread, the error message, and the location in the code where
	the panic occurred, as file:line:character.

	Here, the code contained at the flagged location is in fact the panic!. 
	Sometimes, it might not be our panic! but a panic! included by code we have
	used in our project. We can use bcktraces to follow the chain of function
	calls to determine where our program panics.

Using a panic! Backtrace:

	We're going to induce a panic in code from another library instead of
	calling it directly. This isn't too hard:*/

	fn main() {
		let v = vec![1, 2, 3];

		v[99];
	}

	/*

	This is an array-out-of-bounds error, which panics. In C, reading outside
	of bounds produces undefined behavior. You get _something_, but it's not
	guaranteed to be relevant to what you needed, and can cause security
	vulnerabilities if a user can influence this behavior. Because of this,
	Rust will simply not allow you to perform such a buffer overread.

	As such, here is the error Rust produces when we try to run this code:

	$ cargo run
	   Compiling panic v0.1.0 (file:///projects/panic)
	    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
	     Running `target/debug/panic`
	thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
	note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

	The error points out line 4 of main.rs. The next line tells us to set our
	environment variables to see a backtrace of the error. A backtrace is just
	a listing, in order, of all the functions on the call stack. The way we use
	a backtrace is to start at the top and work down until we arrive at some
	code that we have written; that is where we have introduced erroneous code.

	Backtraces are REALLY USEFUL so it's important to set this up. Setting an
	environment variable looks like setting export [varname]=[value] in your
	rcfile of choice. For me, it's .zshrc, and the line looks like
	export RUST_BACKTRACE=1. If we set this environment variable and rerun the
	code, we get a different set of errors:

	$ RUST_BACKTRACE=1 cargo run
	thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
	stack backtrace:
	   0: rust_begin_unwind
	             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/std/src/panicking.rs:584:5
	   1: core::panicking::panic_fmt
	             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/panicking.rs:142:14
	   2: core::panicking::panic_bounds_check
	             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/panicking.rs:84:5
	   3: <usize as core::slice::index::SliceIndex<[T]>>::index
	             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/slice/index.rs:242:10
	   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
	             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/slice/index.rs:18:9
	   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
	             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/alloc/src/vec/mod.rs:2591:9
	   6: panic::main
	             at ./src/main.rs:4:5
	   7: core::ops::function::FnOnce::call_once
	             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/ops/function.rs:248:5
	note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

	So, what do we see here? This is a list of every function on the stack at
	the time the panic occurred, where function 0 is the top of the stack. We
	see Rust called rust_begin_unwind because the panic occurred, and this is
	the last thing that happens before the program shuts down. Under that, we
	have panic_fmt, which seems like it's related to the panic! printing out
	its error message? Then we have panic_bounds_check, which is probably part
	of the code that handles array accesses, and the thing we violated in our
	code. This isn't code that we wrote, though, so we have to keep going; it
	may be able to give us an insight into what we did wrong, but it isn't 
	something that we can change directly. Functions 3, 4, and 5 have to do
	with the actual implementation of allocating and indexing into the array.
	Finally, function 6 is where we have introduced our panic: it occurs at
	line 4, character 5, in src/main.rs. So we know where the error is and can
	take a look at it! Below that is FnOnce::call_once, which I can only
	imagine is some function in the core of Rust that makes 1 call to main().

Recoverable Errors with Result:

	We can encounter many errors without requiring a full program restart. To
	handle errors in this way, we need to identify, interpret, and respond to
	that error state. The Book gives an example of failing to find a file; we
	might want to make the file with some reasonable defaults instead of
	crashing.

	The Result type is very useful for this. Recall the definition of Result:*/

	enum Result<T, E> {
		Ok(T),
		Err(E),
	}

	/*

	T and E represent generic types here. T represents the type of the value
	that gets returned on successful function execution; E represents the type
	of error that happens on unsuccessful function execution. These generic
	type parameters provide us a lot of flexibility for programming error
	handling with Result.

	Let's take a look at some code that produces Results:*/

	use std::fs::File;

	fn main() {
		let greeting_file_result = File::open("hello.txt");
	}

	/*

	The return type of File::open() is a Result<T, E>. The generic types get
	filled in by the possible return types of File::open(); T, as the success
	type, is replaced with std::fs::File, and E, as the failure type, is filled
	in with std::io::Error.

	If File::open() succeeds, the value in greeting_file_result is the Ok
	variant of the Result enum, holding the std::fs::File we requested. If it
	does not, it is an instance of Err that gives us information about the
	failure.

	In order for our code above to be useful, we need to handle these variants.
	We can do that with a match, the same way we handled Option in the past:*/

	use std::fs::File;

	fn main() {
		let greeting_file_result = File::open("hello.txt");

		let greeting_file = match greeting_file_result {
			Ok(file) => file,
			Err(error) => panic!("Problem opening file: {:?}", error),
		};
	}

	/*

	Note that, like Option, Result and its variants are brought into scope by
	the prelude, so we do not explicitly need to bring them into scope or
	specify Result:: in the match arms.

	It should be clear that if File::open() succeeds, we get the file back,
	and if it fails, we panic and print the error. If we encounter this error,
	here is what is printed:

	$ cargo run
	   Compiling error-handling v0.1.0 (file:///projects/error-handling)
	    Finished dev [unoptimized + debuginfo] target(s) in 0.73s
	     Running `target/debug/error-handling`
	thread 'main' panicked at 'Problem opening file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:8:23
	note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

	Like the errors before, this one tells us exactly what happened. The
	backtrace would also provide more detailed information.

Matching on Different Errors:

	Our code above will panic with the same error regardless of the type of
	that error; this is not always the behavior we would like to exhibit,
	however. We can use error types to handle different scenarios differently.
	The following example showcases using ErrorKind to create a default file
	if the file we are looking for is not found:*/

	use std::fs::File;
	use std::io::ErrorKind;

	fn main() {
		let greeting_file_result = File::open("hello.txt");

		let greeting_file = match greeting_file_result {
			Ok(file) => file,
			Err(error) => match error.kind() {
				ErrorKind::NotFound => match File::create("hello.txt") {
					Ok(fc) => fc,
					Err(e) => panic!("Problem creating file: {:?}", e),
				},
				other_error => {
					panic!("Problem opening file: {:?}", other_error);
				}
			}, //comma as internal match is arm of external match
		}; //semicolon for first match
	}

	/*

	Let's break down what is happening here. As before, we get either an Ok
	or Err state from File::open(). If we get the Err state, though, we check
	what type of error it is. If it's a NotFound error, we attempt to make the
	file at the desired path. The function we use for this, File::create(),
	also returns a Result<T, E>, so we can match against that. If we succeed at
	creating our file, we return the filepath; if we don't, we stop trying to
	fix the problem and panic. In the case that we encounter an error that we
	did not specifically enumerate, then we panic within a catchall arm.

Alternatives to Using match with Result<T, E>:

	match is great, match is cool, match can produce some wonky looking code.
	Instead of match, we can use closures instead; many of the methods defined
	on Result<T, E> use closures to make code more concise. The Book provides
	an example of code using closures to implement the same functionality as
	the snippet above:*/

	use std::fs::File;
	use std::io::ErrorKind; // note we have the same use statements for closure

	fn main() {
		let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
			if error.kind() == ErrorKind::NotFound {
				File::create("hello.txt").unwrap_or_else(|error| {
					panic!("Problem creating the file: {:?}", error);
				})
			} else {
				panic!("Problem opening the file: {:?}", error);
			}
		});
	}

	/*

	This code is a lot cooler. It can be a little difficult to follow the flow
	of the code, as the braces are very strange here, but I think this is in
	fact more concise than the above. unwrap_or_else allowing us to use the Ok
	variant and provide inline code for handling the Err variant is very cool.

Shortcuts for Panic on Error: unwrap and expect:

	Using match works, but can be verbose and noncommunicative of intent. There
	are many methods defined on Result<T, E> to alleviate this. unwrap() is a
	shortcut method implemented like the match in the snippet before the last
	one. If the Result is Ok, unwrap gives it to us, and if it's Err, unwrap
	will call panic! for us. Here is how that might look:*/

	use std::fs::File;

	fn main() {
		let greeting_file = File::open("hello.txt").unwrap();
	}

	/*

	A lot cleaner, right? This will give us an error like this if our code
	fails:

	thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os {
	code: 2, kind: NotFound, message: "No such file or directory" }',
	src/main.rs:4:49

	We can also use expect() instead of unwrap(). expect() takes a string
	parameter, which it uses as the error message in the failure case. Using
	expect() might look something like this:*/

	use std::fs::File;

	fn main() {
		let greeting_file = File::open("hello.txt")
			.expect("hello.txt was not found, and should be created.");
	}

	/*

	Usually, using expect() is a good idea, as communicating why you expect an
	error may have occurred is a good way to solve the problem if it occurs.

Propagating Errors:

	We do not need to always handle an error as soon as we encounter it. What
	we may choose to do instead is to hold onto the error and pass it off to
	another section of code devoted to handling errors. This is referred to as
	"propagating the error." This is good because it gives us a lot more
	flexibility in how we handle errors, since we can change scope and context
	before doing so.

	The next listing shows a function that reads a name from a file. If the
	file does not exist or can't be read, the function returns those errors
	to the caller for handling:*/

	use std::fs::File;
	use std::io::{self, Read};

	fn read_username_from_file() -> Result<String, io::Error> {
		let username_file_result = File::open("hello.txt");

		let mut username_file = match username_file_result {
			Ok(file) => file,
			Err(error) => return Err(error),
		};

		let mut username = String::new();

		match username_file.read_to_string(&mut username) {
			Ok(_) => Ok(username),
			Err(e) => Err(e),
		} //Note lack of semicolon as we are returning the selected arm, here.
	}

	/*

	So, what is our code doing? Let's start from the function signature. First,
	we define the function and its return type; the return type is a Result,
	with the generics replaced by concrete types String and io::Error.

	If the function succeeds without issue, we get a file at line 338 and then
	it is parsed into a string and returned on line 345. If either of these
	actions fail, we return the appropriate error, instead. 

	The code which calls this function either receives a String, or an error,
	based on the variant we end up with. It's up to the caller to determine
	how to handle each of these cases.

A Shortcut for Propagating Errors: The ? Operator:

	Let's look at some code with identical functionality to the above snippet,
	but with much more clarity:*/

	use std::fs::File;
	use std::io::{self, Read};

	fn read_username_from_file() -> Result<String, io::Error> {
		let mut username_file = File::open("hello.txt")?;
		let mut username = String::new();
		username_file.read_to_string(&mut username)?;
		Ok(username) //returned value if all ? operations succeed
	}

	/*

	Knowing that this code has the same functionality as the previous snippet,
	it should be clear what the ? operator is doing. If not, I will explain:

	fn operation_with_fail_state() -> Result<T, E> {
		let result = operation();
		let returned_variant = match result {
			Ok(T) => Ok(T),
			Err(E) => return Err(E)
		}
	}

	If the potentially-failing operation does fail, we immediately exit the
	function and give back the error. Otherwise, we keep the Ok state and
	continue execution of the function. There is one peculiarity: Errors
	returned through ? run through the from function, defined in the From trait
	in the standard library. This trait allows conversion of values from one
	type to another. For ?, this is used to turn the error type ? provides into
	the same type as the error the function calling ? wants to return. 

	For example, if we change read_username_from_file to return a custom error
	type named OurError, we can then define impl From<io::Error> for OurError.
	This will construct an instance of OurError from an io::Error, and then the
	? operator calls this From<io::Error> and give us a value of type OurError
	without us having to put that conversion into the code calling ?.

	We can shorten the code further by chaining the calls:*/

	use std::fs::File;
	use std::io{self, Read};

	fn read_username_from_file() -> Result<String, io::Error> {
		let mut username = String::new();

		File::open("hello.txt")?.read_to_string(&mut username)?;

		Ok(username)
	}

	/*

	And we've gotten down to just 3 lines, each of which is stating pretty
	clearly what it's trying to do! 

	For this specific operation, we can actually make things even shorter:*/

	use std::fs;
	use std::io;

	fn read_username_from_file() -> Result<String, io::Error> {
		fs::read_to_string("hello.txt")
	}

	/*

	It was a one-liner all along. That's just for this specific use case of
	reading in the string, however; the concepts surrounding ? are still in
	general extremely useful.

Where the ? Operator Can Be Used:

	The ? operator can only be used in functions whose return type is
	compatible with the value ? is used on. This is because ? is defined to
	perform an early return out of the function, which means any function using
	? must return some kind of Result<T, E>. Let's look at code using ? that
	won't compile:*/

	use std::fs::File;

	fn main() {
		let greeting_file = File::open("hello.txt")?;
	}

	/*

	Here, we attempt a function with a failure state. Because this main does
	not define a return type, its return type is implicitly (), the unit type,
	and not Result<T, E>. Because we're using ? in the scope of main, main must
	return a Result<T, E> in order for this code to be valid. Here is the
	error we get from this code:

	$ cargo run
	   Compiling error-handling v0.1.0 (file:///projects/error-handling)
	error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
	 --> src/main.rs:4:48
	  |
	3 | fn main() {
	  | --------- this function should return `Result` or `Option` to accept `?`
	4 |     let greeting_file = File::open("hello.txt")?;
	  |                                                ^ cannot use the `?` operator in a function that returns `()`
	  |
	  = help: the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `()`

	For more information about this error, try `rustc --explain E0277`.
	error: could not compile `error-handling` due to previous error

	This error actually gives us some great information: We can use ? on both
	Result AND on Option! What great flexibility!

	To solve this error, we either need to change the return type of our 
	function to match the return types of ?, or use a match block to handle the
	different variants of ? appropriately.

	Note that using the ? on a function returning Option<T> works pretty much
	identically to using it on a function returning Result<T, E>, except we do
	not need to implement the From:: trait on an error type, as the error type
	is simply None. Also note that if you use ? on a function returning Result,
	you cannot also use ? on a function returning Option in that same scope.

	We can use ? directly in the scope of main, if we change its return type.
	main() cannot freely choose its return type; there are limitations, but we
	can return Result<(), E>. The Book shows a snippet for this:*/

	use std::error::Error;
	use std::fs::File;

	fn main() -> Result<(), Box<dyn Error>> {
		let greeting_file = File::open("hello.txt")?;

		Ok(())
	}

	/*

	Box<dyn Error> is a trait object, which is talked about more in Chapter 17.
	In the interim, it just means "any kind of error." So, what we're saying
	with this main function is that if an error occurs, we will return it, no
	matter what kind of error it is. This is a catchall pattern for any kind
	of error ? could return.

	If main does not return any error, it will return 0 instead. If it does
	return an error, then we get a nonzero value. Main can return any types
	implementing std::process::Termination, which contains a function report
	which returns an ExitCode. To look into implementing types with this
	functionality, check out the standard library documentation for 
	std::process::Termination.

To panic! or Not to panic!:

	How do we decide whether to panic! or to return a Result? When code panics,
	the error is unrecoverable. If we force our code to panic, we are making
	the decision that the situation is unrecoverable, taking that agency away
	from calling code. If we return a Result, we give the caller the agency to
	determine how they would like to handle the situation. This makes any code
	using your code much more flexible, as the caller could just choose to
	panic anyways, but they do not always have to. Results are thus a better
	default choice than panics.

	In examples, prototypes, and tests, panics are more appropriate. We'll talk
	briefly about why, and then look at situations where the compiler cannot
	tell that failure is impossible, but you as the programmer can.

Examples, Prototype Code, and Tests:

	If we are writing an example to illustrate a concept, robust error-handling
	code can make things bulky and take away from the concept on display. In
	these scenarios, it is understood that calling a function which may fail
	should have proper error handling when implemented in production contexts.

	If we're prototyping, unwrap and expect are useful as markers for where
	better error handling should be introduced once the main functionality of
	that piece of code is implemented.

	If a method fails in a test, the test is failed and we want to know about
	it. In that case, panicking immediately is the correct choice.

Cases in Which You Have More Information Than The Compiler:

	Calling unwrap or expect is fine in situations where you have logic your
	code represents which can never fail, but need to call a function returning
	a Result anyways. If we can manually inspect the code to confirm it will
	never return Err, we can just handle the result and pass to expect a string
	detailing why the function should never fail. The Book gives an example:*/

	use std::net::IpAddr;

	fn main() {
		let home: IpAddr = "127.0.0.1"
			.parse()
			.expect("Hardcoded IP address should be valid.");
	}

	/*

	Here, we know before compilation that this IP will always be valid. Because
	parse returns a Result, we still need to unwrap it, and because we know
	that this IP will always be valid, we can do so with expect and write the
	reason this process will never fail inside of it. In the case the IP came
	from a user, we would want to handle this with Result in a better way, but
	because we know it isn't, expect is fine.

Guidelines for Error Handling:

	It is advisable to panic when your code can end up in a bad state. A "bad
	state" is when some fundamental thing is wrong with your code: a guarantee,
	assumption, or invariant that your code takes as its logical basis has been
	violated. Examples include invalid values, contradictory values, or missing
	values, and one or more of the following:

		-	The bad state is unexpected, as opposed to something you may
			foresee happening, like an invalid data format.

		-	Your code after this point relies on its data being valid, and
			cannot afford to check errors at every step.

		-	There is not a good way to encode information about the error based
			on the task you require. This is talked about in more detail in
			Chapter 17.

	If our code is called with bad values, we often want to return an error so
	the calling code can figure out what they want to do. In cases where doing
	so may compromise the stability or security of the program, it might be
	better to call panic! and tell the dude screwing up your code to fix it.
	In a similar fashion, panic! is often appropriate when calling external
	code that may return a bad state that you cannot fix. 

	When failure is commonly expected or foreseen, a Result should be used. In
	these cases, returning the Result indicates to the caller that there is a
	potential error and they must provide handling code.

	If our code performs an operation that could put a user at risk if called
	with invalid values, we should validate the values and panic if that fails.
	This is for code safety reasons, similar to how at the start of this
	chapter we talked about how buffer overreads are handled in Rust.

	Functions often have "contracts," which are just conditions under which
	their behavior is guaranteed. Usually these are inputs, and it is generally
	valid to panic if the contract is violated. This is because violating the
	contract is a problem with calling code, not your code, and must be handled
	by the callers as such. A function's contracts should be explained in its
	API documentation.

	We can reduce the amount of error checks we have to do, and thus the amount
	of errors we need to think about propagating, if we design our functions
	intelligently. For example, instead of accepting an Option as parameter and
	having to check against both Some<T> and None, we can just specify the type
	and our code does not have to worry about None anymore; that's for the
	caller to handle. A similar example would be to use u32 instead of i32 in
	contexts where negative values would mean something has gone wrong: the
	compiler will not compile code violating this type specification, and thus
	the error cannot occur in our code and has implicitly been handled.

Creating Custom Types for Validation:

	We can use Rust's type system to ensure we have a valid value in a more
	complex way: custom types for validation. In Chapter 2, we asked the user
	for a guess between 1 and 100, but never validated their input as being
	within the correct range. It would be useful to have code that explains the
	invalid state to the user based on how it is invalid: a number might be out
	of range, but alphabetical characters are just the wrong type entirely.

	We can do this by parsing the guess as an i32 instead of a u32, and then
	check against the number being in range:*/

	loop {
		//stuff

		let guess: i32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue, //not using ? because we want to continue
		};

		if guess < 1 || guess > 100 {
			println!("The secret number will be between 1 and 100.");
			continue;
		}

		match guess.cmp(&secret_number) {
			//stuff
		}
	}

	/*

	Here, we check if our value is in range, and if not, we inform the user of
	the error and start the next iteration to get another guess. After the if,
	we can proceed with our comparisons because we know the potential states
	of guess.

	This solution works but is not good. If it was critical that this program
	only operate on numbers in this range, and had many functions with this
	requirement, then having a check like this everywhere would be tedious and
	probably impact performance if we did it enough.

	Instead, we can make a new type and put the validations in a function to
	create an instance of the type rather than putting the validations
	everywhere. It then becomes safe for functions to use the new type as
	parameters and confidently use the values passed to them. The Book gives an
	example of this for a Guess type:*/

	pub struct Guess {
		value: i32,
	}

	impl Guess {
		pub fn new(value: i32) -> Guess {
				if value < 1 || value > 100 {
					panic!("Guess value must be between 1 and 100, got {}", value);
				}
	
			Guess { value }
		}

		pub fn value(&self) -> i32 {
			self.value
		}
	}

	/*

	We've moved the error-checking into our new() function so as to only even
	provide the type if it is in a guaranteed valid state. 

	The conditions under which Guess::new() might panic should be discussed in
	its API documentation. This is an example of violating a function's
	contract.

	We also implement a getter, because this type is really just a wrapper for
	an i32. We have to do this because the value field in Guess is private.
	Because we do not provide a setter, and the only way to initialize a Guess
	variable is through Guess::new(), which fails on invalid state, we have
	guaranteed the validity of any Guess object that exists. Any function that
	relies on this guarantee can then take a Guess parameter and not have to
	worry about implementing its own error-checking functionality.

Summary:

	Rust provides very robust tools for error checking. panic! forces us to
	stop execution on unrecoverable states, and Result allows us to detect and
	handle or propagate errors which are recoverable. unwrap, expect, and ? are
	all good ways to increase clarity of code, and in cases where these tools
	do not provide the exact functionality we desire, we can always write a
	match instead. 

	I like this stuff, a lot. I thought Option and Result were cumbersome, if
	useful, but this chapter has given me a lot of powerful and convenient
	tools to complement the power of Option and Result.