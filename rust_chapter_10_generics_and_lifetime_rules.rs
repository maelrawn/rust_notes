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
