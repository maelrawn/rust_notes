/*

These are my notes on Chapter 10 of The Book.

Generic Types, Traits, and Lifetimes:

	Generic Types are, well, generic types. Rust makes defining functions which
	accept generic types very easy; the syntax is:

		fn foo<T>(var_name: T) -> T

	We can do the same in other places, too. Consider this struct:*/

		struct Point<T>{
			x: T,
			y: T,
		}

	/*
	This allows us to instantiate a Point object with arbitrary type x and y.
	Note that because we only declared one type, T, x and y must be the same type
	regardless of what type that is. We could extend this out arbitrarily like so:
	*/
		struct Point<T, U>{
			x: T,
			y: U,
		}

	/*
	Examples of these generically-typed tuples are abundant in rust. Consider
	Option and Result:*/

		enum Option<T> {
	    Some(T),
	    None,
		}

		/*
		Here, generic types are used to represent an optional value (Some) or no
		optional value (None).*/

		enum Result<T, E> {
	    Ok(T),
	    Err(E),
		}
		
		/*
		Here, generic types are used to represent a success state (Ok) and a fail
		state (Err).

		We can also implement methods on generic types. Consider the following:*/

		impl<T> Point<T>{
			fn x(&self) -> &T{
				&self.x
			}
		}

		/*
		This allows us to return element x from Point. If we didn't use generics
		here, we would have to specify the return type, which is an issue if we're
		using generics to hold x! We can also do some type gymnastics if we want:*/

		struct Point<X1, Y1> { //Declare point with generic types X1 and Y1
	    x: X1,
	    y: Y1,
		}

		impl<X1, Y1> Point<X1, Y1> { 
		    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
		        Point {
		            x: self.x,			//Declare a function mixup that accepts another
		            y: other.y,			//Point, with potentially mismatched types!
		        }
		    }
		}

		fn ex_4() {
		    let p1 = Point { x: 5, y: 10.4 };
		    let p2 = Point { x: "Hello", y: 'c' };

		    let p3 = p1.mixup(p2);

		    println!("p3.x = {}, p3.y = {}", p3.x, p3.y); //p3 now holds an int and
		}																									//a string literal.

/*
Traits:

	Traits are similar to interfaces, but with some differences. Explicitly, it
	defines functionality a particular type has, and can have in common with 
	other types. We can also use trait bounds to indicate that a generic type
	should have some functionality in order to be used in a given context.

	For example, consider a trait that asks text items to summarize themselves:*/

	pub trait Summary{
		fn summarize(&self) -> String;
	}

	/*
	Here, pub allows the trait to be implemented by packages depending on this
	package, and then fn summarize(&self) -> String is the signature that must be
	provided for. Here's what implementing might look like:*/

	pub struct NewsArticle {
	    pub headline: String,
	    pub location: String,
	    pub author: String,
	    pub content: String,
	}

	impl Summary for NewsArticle {
	    fn summarize(&self) -> String {
	        format!("{}, by {} ({})", self.headline, self.author, self.location)
	    }
	}
	
	/*
	Note it is declared outside of the struct body. We can also provide a default 
	implementation in the trait's definition. It is used by providing an impl
	block with an empty body, i.e. impl Summary for NewsArticle {}.
	When we use traits, we have to bring them into scope if we did not declare
	them ourselves. An example:

	use aggregator::{Summary, NewsArticle}

	We can also mix default and custom implementations. The rust book gives this
	example:*/

	pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    	}
	}
	
	/*
	Here, summarize_author() must be defined by the implementor, but summarize()
	does not need to be, as it already defines its own functionality using the 
	requested functionality.

	Traits can be used as parameters, similarly to how interfaces can be used as
	parameters in java. Using our previous examples of Summary:*/

	pub fn notify(item: &impl Summary) {
    	println!("Breaking news! {}", item.summarize());
	}

	/*
	Here, instead of specifying a type, we specify that the given item must 
	implement Summary, so we can make sure we can call .summarize() in notify().
	We can extend this into generic typing. We may specify that generic types in
	some context require an implementation for a given trait. For example,
	we can rewrite notify() as such:*/

	pub fn notify<T: Summary>(item: &T) {
    	println!("Breaking news! {}", item.summarize());
	}

	/*
	This implementation is functionally identical to the previous implementation.
	However, there is some nuance to it:*/

	pub fn notify(item1: &impl Summary, item2: &impl Summary) {
		//item1 and item2 can have different types as long as they impl Summary
	}

	pub fn notify<T: Summary>(item1: &T, item2: &T) {
		//item1 and item2 must be the same type!
	}

	pub fn notify<T: Summary, U: Summary>(item1: &T, item2: &U){
		//this is identical to the first form, but written with trait bounds.
	}

	/*
	We may also specify multiple requisite traits with a + operator. i.e. we may
	make a declaration like:*/

	pub fn notify(item: &(impl Summary + Display)) {}

	//or

	pub fn notify<T: Summary + Display>(item: &T){}

	/*
	A natural question to ask is, what happens when I have several variables with
	different trait bounds? How does that look when I write it out? The answer is
	awful, so we are given a tool to remedy this: the where block. We can turn
	this:*/

	fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

	//into this:

	fn some_function<T, U>(t: &T, u: &U) -> i32
	where
    T: Display + Clone,
    U: Clone + Debug,
	{}

	/*
	which is much more readable. We can also use this syntax to specify return
	types implementing some trait. We do so in the following function, but note
	we can only return one type.*/

	fn returns_summarizable() -> impl Summary {
		Tweet {
			username: String::from("horse_ebooks"),
			content: String::from(
				"of course, as you probably already know, people",
			),
			reply: false,
			retweet: false,
		}
	}

	/*
	This feature is useful when using closures and iterators; these language
	features return types that are not named by the programmer, so being able
	to refer to them by functionality is useful.

	We can also perform conditionals with type deduction using trait bounds.
	The book provides an example of conditionally implemented methods for types
	satisfying the criteria specified by the trait bounds. 

	Here, we implement Pair::new(x, y). Then, for pairs of types implementing
	Display + PartialOrd, we define a new function, cmp_display. For types that
	do not have both of these traits, this function will not exist.*/

	use std::fmt::Display;

	struct Pair<T>{
		x: T,
		y: T,
	}

	impl<T> Pair<T> {
		fn new(x: T, y: T) -> Self {
			Self { x, y } 
		}
	}

	impl<T: Display + PartialOrd> Pair<T> {
		fn cmp_display(&self) {
			if self.x >= self.y {
				println!("The largest member is x = {}", self.x);
			} else {
				println!("The largest member is y = {}", self.y);
			}
		}
	}

	/*
	We can also implement a trait for any type implementing some other trait;
	i.e. traits conditional on underlying traits. This sounds like a quick way
	to build a lot of functionality, and the book says the standard library
	does a lot of this. This feature is called blanket implementation, and it
	looks like this:*/

	impl<T: Display> ToString for T {
		//the body would be available on any type implementing Display!
	}

	let s = 3.to_string() //we can do this because ints implement Display

	/*
	The reason we do this in this manner is because it lets the compiler figure
	out what is and is not valid code, and stops us from running invalid code.
	This moves a lot of development time from bugfixing after compilation and 
	testing into writing cleaner code before compiling.


Validating References with Lifetimes:

	Most of the time, lifetimes are implicit and follow the borrowing rules
	established in chapter 4. We open a scope, create a variable or reference,
	and then close the scope, dropping the relevant items. This gives those 
	items an implicit lifetime of the duration our program stays in that scope.
	However, consider the following:*/

	fn main() {
		let r;

		{//note that we can actually just start a new scope because we want to.
			let x = 5;
			r = &x;
		}//Here, x is dropped, but r belongs to the outer scope and is not!

		println!("{:?}", r);//r now references missing data
	}

	/*
	The compiler will whine and say that the borrow doesn't live long enough,
	which is true. This is an example of variables having conflicting lifetimes.
	If the compiler allowed this code to run, r would reference deallocated
	data and produce undefined behavior, which we don't want.

The Borrow Checker:

	The compiler has a borrow checker that examines scopes to determine whether
	all borrows are valid. Here is the previous code, but with lifetimes
	annotated:*/

	fn main() {
	    let r;                // ---------+-- 'a
	                          //          |
	    {                     //          |
	        let x = 5;        // -+-- 'b  |
	        r = &x;           //  |       |
	    }                     // -+       |
	                          //          |
	    println!("r: {}", r); //          |
	}                         // ---------+

	/*

	Here, r has a lifetime of 'a, and x has a lifetime of 'b. Since r holds a
	reference to x, but has a longer lifetime, the compiler gets angry at us.
	To fix the code in this case, we can restructure our program:*/

	fn main() {
	    let x = 5;            // ----------+-- 'b
	                          //           |
	    let r = &x;           // --+-- 'a  |
	                          //   |       |
	    println!("r: {}", r); //   |       |
	                          // --+       |
	}                         // ----------+

	/*

	Here, the lifetime of x is 'b, and the lifetime of r is 'a. r can thus
	reference x, as the compiler knows the reference will always be valid.

Generic Lifetimes in Functions:

	Let's write a function that returns the longer of two string slices. The
	function will take two slices as parameters and return a single slice.
	After we implement it, the following code should print "The longest string
	is abcd".:*/

	fn main() {
		let string1 = String::from("abcd");
		let string2 = "xyz";

		let result = longest(string1.as_str(), string2);
		println!("The longest string is {}", result);
	}

	/*

	Note that we want to take slices, which are references. If we passed in
	the Strings themselves, the function would take ownership, which we don't
	want. If we try to write this function as follows, we fail to compile it:*/

	fn longest(x: &str, y: &str) -> &str {
		if x.len() > y.len() {
			x
		} else {
			y
		}
	}

	/*

	Trying to compile this code produces the following error:

	$ cargo run
	   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
	error[E0106]: missing lifetime specifier
	 --> src/main.rs:9:33
	  |
	9 | fn longest(x: &str, y: &str) -> &str {
	  |               ----     ----     ^ expected named lifetime parameter
	  |
	  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
	help: consider introducing a named lifetime parameter
	  |
	9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	  |           ++++     ++          ++          ++

	For more information about this error, try `rustc --explain E0106`.
	error: could not compile `chapter10` due to previous error

	Before reading more into the book, let's think about this line:

		help: this function's return type contains a borrowed value, but the 
		signature does not say whether it is borrowed from `x` or `y`

	So, we know we're working with references, and the function takes ownership
	of those references, meaning the reference not returned by the function
	ends up getting dropped. Thus, the two references have different lifetimes
	and the compiler cannot be sure if the returned reference will be valid!

	Let's see what the book says:

	It says that my insight is correct! Feels good.

Lifetime Annotation Syntax:
		
	Lifetime annotations do not change how long the references live for. What
	they do provide is a description of the relationships of lifetimes of
	multiple references to one another, without affecting those lifetimes. The
	same way functions can accept any type when its signature indicates a
	generic type parameter, functions can accept references with any lifetime
	by specifying a generic lifetime parameter.

	Lifetime annotations have the syntax 'x. i.e. they start with an ', and are
	usually lowercase and very short. Usually, we start with 'a as our first
	lifetime annotation. We place the annotation after the & of a reference.
	Some examples include:*/

	&i32		//	a reference
	&'a i32		//	a reference with an explicit lifetime
	&'a mut i32	//	a mutable reference with an explicit lifetime

	/*

	One annotation doesn't do anything on its own, as they're intended to tell
	the compiler how two or more references are related.

Lifetime Annotations in Function Signatures:

	So we know what these things are, now, but how do we use them? To do so, we
	need to declare the generic lifetime parameters in our function signature.
	This is similar to declaring generic type parameters.

	In the following example, we exercise this functionality in order to
	express the following idea: the returned reference will be valid as long
	as both of the parameters are valid. This is the relationship between the
	lifetime of the input variables and the lifetime of the output variable.*/

	fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
		if x.len() > y.len() {
			x
		} else {
			y
		}
	}

	/*

	This code is now functional and does what we wanted to achieve with our
	broken fn main from earlier. Let's break down how it accomplishes that:

	We have specified a lifetime 'a in our function signature. By denoting both
	x and y as &'a, we are saying they are references which each live at least
	as long as lifetime 'a. The function signature also says that the returned
	reference also lives at least as long as lifetime 'a. What does this mean?

	Say we have 2 different input lifetimes (exercise to the reader: come up
	with scenarios where this is not true. I think it's when multiple values
	are referenced from a single data structure) between two parameters. Is
	there any information we know must be true about those lifetimes? Well, if
	we know that one of these lifetimes is longer than the other, then we can
	establish a greatest lower bound for the lifetime of the longer variable,
	and a minimum upper bound for the lifetime of the shorter variable. Since
	we're trying to make guarantees about memory safety, we need something
	concrete. If we establish a minimum upper bound on the lifetime of the
	shorter-lived variable, there is still some nonzero interval where it is
	an invalid reference. However, if we establish a greater lower bound on the
	lifetime of the longer-lived reference, there is no time in that interval
	in which the reference will be invalid. So when we specify that 2+ values
	in our functions have a shared lifetime, we are saying "these each live for
	at least as long as the shortest lived of them all."

	Note that these lifetimes only need to be specified in the function
	signature. They become part of the contract of the function, which we spoke
	about before. Since lifetime specifications are part of the generics suite,
	when we subsitute concrete values for x and y, our lifetime 'a also gets
	substituted with a concrete lifetime, equivalent to min(life(x), life(y)).

	Let's look at what happens when we try to use the function we've written:*/

	fn main() {
		let string1 = String::from("abcd");

		{
			let string2 = String::from("xyz");
			let result = longest(string1.as_str(), string2.as_str());
			println!("The longest string is {}", result);
		}
	}

	/*

	Here, string1 and string2 clearly have different lifetimes. The borrow
	checker is ok with it because we specified the generic lifetime parameter.

	If we rearrange the scopes like this, we run into an error:*/

	fn main() {
		let string1 = String::from("abcd");
		let result;
		
		{
			let string2 = String::from("xyz");
			let result = longest(string1.as_str(), string2.as_str());
		}
		println!("The longest string is {}", result);
	}

	/*

	Here, our code fails. string2 has a lifetime of only the inner scope, while
	string1 and result are declared in the outer scope, and result is used
	after the end of the inner scope. Because x, y, and the returned reference
	all have lifetimes of 'a, which is equivalent to the lifetime of a variable
	declared in the inner scope, the returned reference does not live past that
	inner scope. So result actually becomes bound to data that will be dropped
	before println! requests that data, and our program fails to compile.

	Here is the error in full:

	$ cargo run
	   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
	error[E0597]: `string2` does not live long enough
	 --> src/main.rs:6:44
	  |
	6 |         result = longest(string1.as_str(), string2.as_str());
	  |                                            ^^^^^^^^^^^^^^^^ borrowed value does not live long enough
	7 |     }
	  |     - `string2` dropped here while still borrowed
	8 |     println!("The longest string is {}", result);
	  |                                          ------ borrow later used here

	For more information about this error, try `rustc --explain E0597`.
	error: could not compile `chapter10` due to previous error

	This error is describing the same thing I did in the previous paragraph.

Thinking in Terms of Lifetimes:

	Lifetime specification is something you do with purpose. As such, the way
	we choose to annotate lifetimes, or if we choose to annotate them, is going
	to depend on the use case we are considering.

	For example, in the following function, we only return the first parameter.
	In this case, it's not necessary to annotate a lifetime on y:*/

	fn longest<'a>(x: &'a str, y: &str) -> &'a str {
		x
	}

	/*

	We don't have to specify a lifetime for y because it has no relationship to
	x or the return value.

	When returning a reference from a function, the lifetime parameter for the
	return type must match the lifetime parameter for one of the parameters. If
	the reference returned does not refer to one of the parameters, it must
	refer to a value created within this function, but then we have a dangling
	reference, which is still no good. For example, this fails to compile:*/

	fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
		let result = String::from("arbitrary string");
		result.as_str()
	}

	/*

	Even though we specified a lifetime for the return type, the lifetime of
	the value that is actually being returned has no relation to the lifetimes
	of x or y.

Lifetime Annotations in Struct Definitions:

	It's time to talk about structs which hold types that are not owned. The
	structs we've talked about already all took ownership of the data that we
	gave them, so we knew the data would remain valid for as long as the struct
	was valid. So if we want to hold something like an &str, we need to provide
	an annotation for this:*/

	struct ImportantExcerpt<'a> {
		part: &'a str,
	}

	fn main() {
		let novel = String::from("Call me Ishmael. Some years ago...");
		let first_sentence = novel.split('.').next().expect("Could not find a '.'");
		let i = ImportantExcerpt {
			part: first_sentence,
		};
	}

	/*

	The only field in this struct is an un-owned value. By declaring the
	lifetime of this value, we are saying that any instance of ImportantExcerpt
	cannot outlive the reference it holds.

	In the code snippet above, ImportantExcerpt is created in main, and holds
	a reference to novel. This data exists before ImportantExcerpt, and they
	go out of scope at the same time, so our use of ImportantExcerpt is valid.

Lifetime Elision:

	So, every reference has a lifetime and we need to specify those lifetimes
	for structs or functions that use them. But in Chapter 4 we looked at this
	function:*/

	fn first_word(s: &str) -> &str {
		let bytes = s.as_bytes();

		for (i, &item) in bytes.iter().enumerate() {
			if item == b' ' {
				return &s[0..i];
			}
		}
		&s[..]
	}

	/*

	Notice anything missing? The lifetime annotations! The reason for this is
	that people got very tired of annotating lifetimes for functions accepting
	a single reference as parameter. The Rust developers just added a way for
	the compiler to deduce this lifetime pattern and insert the annotations on
	its own. This is called a lifetime elision rule. There are more than one of
	these, and the list may expand in the future. 

	The lifetime elision rules do not provide the compiler a way to fully infer
	the necessary lifetimes in all situations. If it can't, it will tell you
	what information you must provide. The compiler follows 3 rules to figure
	out the lifetimes of references without explicit annotations. They apply to
	function definitions and impl blocks.

	Rule one: The compiler assigns a lifetime parameter to each parameter which
	is a reference. This means each unique parameter gets a unique lifetime
	stapled on. 

	Rule two: If there is one input lifetime, then the output lifetime inherits
	this input lifetime.

	Rule three: If there are multiple lifetimes, but one of them is &self or
	&mut self, then the lifetime of self is applied to all output lifetime
	parameters.

	It should be clear how the previous function is assigning its lifetimes:*/

	fn first_word(s: &str) -> &str;

	//first, apply rule 1:

	fn first_word<'a>(s: &'a str) -> &str;

	//now, we see that rule 2 applies:

	fn first_word<'a>(s: &'a str) -> &'a str;

	//so our lifetime elision rules have been applied and our lives are easier.

	/*

Lifetime Annotations in Method Definitions:

	When we implement methods on a struct with lifetimes, we use the same
	syntax as with generic type parameters on line 69 of this file. Where we
	declare and use these parameters depends on whether they're related to the
	struct fields or to the method parameters and return values.

	Lifetime names for struct fields always need to be declared after the impl
	keyword, and then used after the struct's name.

	In method signatures in an impl block, references might be tied to the
	lifetime of references in the struct's fields, or they may be independent.
	The lifetime elision rules also frequently mean we do not need to do the
	lifetime annotation ourselves. Let's look back at ImportantExcerpt:*/

	impl<'a> ImportantExcerpt<'a> {
		fn level(&self) -> i32 {
			3
		}
	}

	// The lifetime parameter declaration and its use after the type name are
	// required, but we're not required to annotate the lifetime of the
	// reference to self because of the first elision rule.

	impl<'a> ImportantExcerpt<'a> {
		fn announce_and_return_part(&self, announcement: &str) -> &str {
			println!("Attention please: {}", announcement);
			self.part
		}
	}

	// Here, the first elision rule is applied, giving each parameter a unique
	// lifetime annotation. Then, the third elision rule is applied, setting
	// the lifetime of the return value to the lifetime of the &self parameter.

	/*

The Static Lifetime:

	There is a special lifetime worth mentioning: 'static. This means the noted
	reference can live for the entire duration of the program. All string
	literals have this lifetime, as they are baked into our binary.

	The book also wants to note: if the compiler suggests making a reference
	'static, think very hard about that before you do so.

Generic Type Parameters, Trait Bounds, and Lifetimes, Oh My!:

	Now, let's see if we can read the following code snippet:*/

	use std::fmt::Display;

	fn longest_with_an_announcement<'a, T>(
		x: &'a str,
		y: &'a str,
		ann: T,
	) -> &'a str
	where
		T: Display,
	{
		println!("Announcement! {}", ann);
		if x.len() > y.len() {
			x
		} else {
			y
		}
	}

	/*

	The recommended formatting is a little strange, but yes. This is a function
	which takes and returns references (and is thus lifetime-annotated), and
	operates on a parameter implementing the Display trait. The function 
	makes an announcement using the parameter ann, which just needs to impl
	std::fmt::Display, and then returns the longer of the two &str params.

Summary:

	This is a lot of information, but these systems are extremely flexible and
	provide you with a lot of ways to reduce code duplication in a performant
	way. In fact, if you look at the standard library, there are a million
	billion functions because many of them are generated from combinations of
	traits and different types implementing those traits differently.

	We're also not done: there is more to be said about trait objects and 
	very complicated lifetime scenarios. Up next, though is testing.