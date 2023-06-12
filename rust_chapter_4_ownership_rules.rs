/*

These are my notes on Chapter 4 of The Book.

Ownership:

	Ownership is the set of rules governing how Rust manages memory. Because Rust
	has robust rules for ownership it can make guarantees of memory-safety without
	requiring the overhead of a garbage collector. 

	Stack Vs. Heap:

		Using the stack is generally faster and more organized than using the heap.
		However, the stack requires you to know the size of the memory request in
		advance. For requests of unknown size, the heap is used. Heap-allocated
		memory access is generally slower because of worse processor caching and
		because you have to follow a pointer to get to the data, which isn't 
		required for the stack. Because the heap necessarily requires pointers to 
		function, we can lose data in it if we don't keep track of what is where.
		The point of ownership is to provide a robust framework to make sure we
		cannot lose anything in there.

	Basic rules of ownership:

		- Every value has an owner.
		- There can only be one owner at a time for a given value
		- When the owner exits scope, the value is dropped. 

		Consider the following: */

fn main() 
{										//s has not been declared so is not yet valid
	let s = "hello";	//s is valid from here on
}										//scope ends, so s is no longer valid

/*
	This example was for a string literal. We know the size of literals when they
	are declared, and because rust variables are immutable, we also know it will
	remain that way. However, things change when we introduce mutable String type:
*/

{
	let mut s = String::from("hello");

	s.push_str(", world!"); // appends literal to string

	println!("{}", )s; // prints string
}

/*
	What allows this type to be mutated where literals cannot be? Well, for one,
	string literals are encoded directly into the program binary. Because we know
	exactly what they are, we can do that! However, for String type,

		-	The memory must be allocated during runtime
		- We need a way to return the memory to the allocator

	We do the first part manually by calling methods like String::from() or
	String::new(). That's not hard; calling functions is very easy. What is hard
	is the second part: garbage collected languages have a portion of the program
	dedicated to finding and freeing unused memory; this has computational
	overhead. In non-garbage-collected languages, there is no overhead incurred
	by the nonexistent garbage collector, but you have to perform this function
	manually, which is hard. Freeing too early or freeing the same memory location
	more than once results in segfaults, forgetting to free at all leads to
	memory leaks, and freeing in an untimely fashion can probably cause issues
	as well. Rust automatically calls <Type>::drop() for all variables in scope
	when it encounters a } character. Let's look at an example of some simple
	types. */

{
	let x = 5; //Declare int x = 5
	let y = x; //Declare int y = x, which makes y = 5.
}

{
	let s1 = String::from("hello"); //Create string s1 = "hello"
	let s2 = s1;										//s2 takes ownership s1's ptr, s1 invalid.
}

/*
	assert(s2 == s1) would fail! What causes this difference? The String type is
	heap-allocated. i.e. the String type itself looks like this:*/
		
		struct String{
			ptr 			//Pointer to heap containing the character array
			len 			//integer
			capacity 	// another integer
		} 

	/*
	So when we assign s2 = s1, we copy these 3 values, but NOT the contents of
	the memory being pointed to. If we were to then try to free s1 and s2, we 
	would run into the aforementioned multi-free error. To combat this, Rust
	applies an ownership rule. Specifically, the rule is this one from before:

		- There can only be one owner at a time for a given value

	So, instead of copying s1 into s2, we actually perform a move instead. This
	makes s1 no longer contain any value of importance and is dropped.

	If we do want a deep-copy functionality, we can do so with .clone(). But wait,
	what about those integers?

	Because we know their size at compile-time and they do not change, they are
	stored on the stack. This makes creating duplicates of them trivial and
	inexpensive. More explicitly, _there is no difference between creating a deep
	copy and a shallow copy for this type._ Types for which this is true have an
	annotation called the Copy trait. These types perform copy assignment like
	we would expect from other programming languages. These types include:

		-	Integer types
		- Booleans
		- Floating point types
		- Characters
		- Tuples containing only Copy-annotated types.

Function calls with Ownership (i.e. the important part): from the website:*/

fn ex_1() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String){ // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

	fn ex_2() {
    let s = String::from("hello");  // s comes into scope

    let s = takes_ownership(s);     // s's value moves into the function...
                                    // ... and so is no longer valid here
                                    // unless we explicitly return it from the
                                    // function! look at takes_ownership below:

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
    println!("{}",s);

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) -> String{ // some_string comes into scope
    println!("{}", some_string);
    some_string
} // Here, our return statement prevents some_string from being dropped due to
	// loss of scope! Note that the last line is actually a return statement.
	// weird, I know.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

/*
	This is all well and good, but explicitly returning the thing we used every
	time we need to use it is far too cumbersome. Even simple functions then need
	to return tuples, which becomes a lot of logical overhead for the programmer.
	So instead we use references:*/
	{
		let s1 = String::from("hello");

		let len = calculate_length(&s1); //We are not transferring ownership!
	}

//This has an important implication, however: we cannot change borrowed values.

	fn ex_3() {
	    let s = String::from("hello");

	    change(&s); //Fails to compile!
	}

	fn change(some_string: &String) {//declare some_string: &mut String to fix
	    some_string.push_str(", world");
	}
	
	/*
	Note we cannot mix the two types of reference. If a mutable reference to an
	object exists, then it must be the single, unique reference to that object,
	within that scope. Examine the following:*/

	{
	let mut s = String::from("hello");

	let r1 = &s; // no problem
	let r2 = &s; // no problem
	println!("{} and {}", r1, r2);
	// variables r1 and r2 will not be used after this point

	let r3 = &mut s; // no problem
	println!("{}", r3);
	}

	/*
	Because we call println! with r1 and r2, they are consumed there. Or, more
	accurately, the function takes ownership of those references to s, and once
	its scope ends, those references are dropped. Once those references stop
	existing, we may assign a new, mutable reference to s without objection.

	Note that because of these rules, the compiler can detect when a reference
	would point to nothing (a "dangling" pointer) and refuse to compile until the
	error is fixed. This is the crux of the whole thing, I think.

	We can also look at ownership in the context of slices. Slices are just the
	python thing where you go [a : b]. The text gives this example: given a
	string s, return the first word of the string. Here is their solution:*/

	fn first_word(s: &String) -> usize {
	    let bytes = s.as_bytes();

	    for (i, &item) in bytes.iter().enumerate() {
	        if item == b' ' {
	            return i;
	      	}
    	}

    	s.len()
	}

	/*
	Let's break down how they're solving the problem, first off. 
	s is converted to bytes to allow direct comparisons of characters with their
	ascii values. Then, .iter() assigns an iterator to the array of bytes, and
	.enumerate() turns the lonely iterator into index-value tuples. When a space
	is encountered, its index is returned; if no space is encountered, the end of
	the string is returned. Nice solution. Why is it bad?

	This returns an integer value that is in no way related to the String. The
	contents of the String could change, and there is no guarantee that the
	information represented by that integer is valid any more. It would be better
	if we had some kind of way to dynamically parse strings. So we do it with
	slices!

	The string slice type is referred to as &str. It's made up of a pointer to a
	byte in a string, and an integer representing a length. Because it's got a
	pointer, that means it's a reference, and because it's a reference, rust does
	a lot of bookkeeping for us, by design! We can't have a mutable and immutable
	reference alive at the same time, so we're guaranteed that the data will 
	remain valid until we don't need it anymore. Slices are not unique to String,
	either; they are a generic type used in combination with a variety of
	collections. 

	I think the above ties together the ownership concepts nicely, and is decently
	convincing as to the strengths of programming in this way.*/