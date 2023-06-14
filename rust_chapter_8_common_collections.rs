/*

These are my notes on Chapter 8 of The Book.

This chapter concerns several really common data structures which are vital for
many algorithms. However, there are many more than the few covered here, and
The Book suggests that you view the documentation at some point.

Storing Lists of Values with Vectors:

	Before I write anything from the chapter, let me make a note here. I have
	tried to do things in rust a few times before reading this book, and using
	collections is nigh impossible if you are not well acquainted with the
	ownership rules, the lifetime rules, and the collections themselves. If you
	are me in the future and think you need to read something here, PAY CLOSE
	ATTENTION.

	Vectors are resizable arrays, effectively. Because of their modularity they
	always come with a lot of methods available for manipulation of the data
	inside.

Creating a New Vector:

	This is something we've covered before in other types. As with those types,
	we simply call:*/

	let v: Vec<i32> = Vec::new();

	/*

	Note the type annotation. A vector is a container which accepts any type as
	a generic, so the compiler cannot successfully deduce its type and requires
	the annotation unless we explicitly declare the vector along with values.
	This is a common enough scenario that Rust provides a macro for it:*/

	let v = vec![1, 2, 3];

	/*

	I haven't covered macros explicitly yet, but I'm told that they are a
	metaprogramming feature that expands code inline during compilation. If so,
	then this probably expands out to something like:

	let mut v: Vec<i32> = Vec::new();
	v.push(1);
	v.push(2);
	v.push(3);

	but I'm not positive. It is cool either way.

Updating a Vector:

	We can push elements into a vector that we have declared:*/

	let mut v  = Vec::new();

	v.push(5);
	v.push(6);
	v.push(7);
	v.push(8); //produces a vector of elements [5, 6, 7, 8].

	/*

	Note that we had to declare the vector as mutable in order to perform this
	operation. On some level and coming from other languages, it seems kind of
	silly: what would we have a collection for if we weren't going to mutate
	it? At least in initializing the collection, surely? But remember, rust
	wants us to think about things in a very structured way. So we call it mut.

	Also notice that we did not have to annotate the type of v, because we pass
	elements to it all of the same type later. The compiler will perform type
	annotation between lines if it can, which is kind of amazing.

Reading Elements of Vectors:

	We can reference vector elements in two ways: indexing or with .get(). 
	These two routes return different values, as shown in the example below:*/

	let v = vec![1, 2, 3, 4, 5];

	let third: &i32 = &v[2]; 	// Indexing returns an immutable reference to
								// the element at that index.
	println!("The third element is {third}");

	let third: Option<i32> = v.get(2);	// .get() returns an Option<&T>. This
										// means we have to unwrap the tuple,
										// and gives us a good opportunity to
										// handle any unexpected behavior.
	match third {
		Some(third) => println!("The third element is {third}");
		None => println!("There is no third element");
	}
						
	/*

	As I sort of noted in the snippet, we are given these two different routes
	for a reason: they handle errors differently. Look at what happens when
	we run this code:*/

	let v = vec![1, 2, 3, 4, 5];

	let does_not_exist = &v[100];		// panics program, causing crash.
	let does_not_exist = v.get(100);	// returns None, allowing graceful
										// handling of error.

	/*

	As noted in the snippet, these have completely different behavior, so we
	can choose the one that makes sense for our use case. If not having some
	element is fatal, we would choose the prior, and if not having some element
	is nonfatal or sometimes expected, we would choose the latter and handle
	the missing value accordingly.

	If we have an immutable reference to an element in the vector, we have to
	be cognizant of the ownership rules. Having an immutable reference to an
	element in the vector means we have an immutable reference to the vector
	itself! Because the vector points to the memory where that reference is
	stored, writing to the vector could potentially write over the address of
	the element we hold the immutable reference to! This is important because
	remember, we cannot have a mutable and immutable reference to an element at
	the same time in any scope. For example, this code will not compile:*/

	let mut v = vec![1, 2, 3, 4, 5];

	let three = &v[2];	// We are now holding the second element of the vector
						// as a reference to a specific memory address managed
						// by that vector; i.e. we have a reference to the
						// vector itself.

	v.push(6); 	// Compilation fails here! Mutating object to which we now
				// hold an immutable reference is illegal.

	println!("The third element is {three}");

	/*

	The book provides an even more compelling argument for why this is illegal
	by design: If the vector needs to allocate more memory, there may not be
	any remaining memory contiguous with the vector. In that case, all of the
	elements need to be moved to a location in memory with enough space for the
	new vector size, and the reference would point to deallocated memory!

	The Rustonomicon page at

	file:///Users/james/.rustup/toolchains/1.67-aarch64-apple-darwin/share/doc/rust/html/nomicon/vec/vec.html

	contains more information about the implementation of vec!.

Iterating over the Values in a Vector:

	When we iterate over the values in a vector, it's often easier to use
	iterators than it is to access things by explicitly indexing them. Here
	are some examples of doing that:*/

	let v = vec![100, 32, 57];
	for i in &v { // referencing the elements of v so we do not consume them
		println!("{i}");
	}

	let mut v = vec![100, 32, 57];
	for i in &mut v {	// again referencing so as not to consume, but mutably
		*i += 50;
	}

	/*

	There is new syntax here: to change the value of the mutable reference, we
	have to use the * dereference operator to get the value out of our 
	reference before we are allowed to += it. This is expanded upon more in
	Chapter 15.

	Iterating over a vector is safe because of the borrow checker. Attempting
	to insert or remove elements in the previous for statements is not legal,
	as it constitutes a violation of the mutability of the held reference.

	Note: Look deeper into the examples at line 155 later. It is not totally
	clear what the &v and &mut v are referring to: are they references to the
	vector, or references to the items within the vector? Can approach by
	implementing in a short file and printing the debug info for each value at
	each step; should show what each type is, and then it should be clear.

Using an Enum to Store Multiple Types:

	Vectors, like arrays, require the same type of item in every slot. This
	can make things difficult if we need a few different types in one vector,
	but we have a tool for this. Remember that enums are a single type which
	can come in the form of many different variants: this means that a vector
	could hold many different enum variants (which themselves hold data of
	differing types!) because those variants are the same enum type. For
	example, this code compiles:*/

	enum SpreadsheetCell {
		Int(i32),
		Float(f64),
		Text(String),
	}

	let row = vec![
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Text(String::from("blue")),
		SpreadsheetCell::Float(10.12),
	];

	/*

	This works if we know the types the vector will hold at compile time. We
	also need to then implement matching against the elements of this vector
	when we access those elements, in order to handle all the possible cases
	of differing types. We cannot use this technique if we do not know all the
	different types we want to place in our vector at compile time. In that
	case, we use a trait object to ensure uniform behavior of the nonuniform
	types.

Dropping a Vector Drops Its Elements:

	Like other values in rust, vectors are freed when they exit scope. All of
	the values contained in that vector are also dropped. The borrow checker
	will complain if you try to reference elements from a dropped vector, so
	be careful.

Storing UTF-8 Encoded Text with Strings:

	The Book starts this section with a reality check: Strings are complicated.
	Rust requires you to perform error checking, and error checking on text or
	text-based data structures can be difficult, strings are complicated, and
	UTF-8 is complicated or at least nontrivial. So pay attention!

What Is a String?:

	When we say string, we mean the string slice str that is usually seen as
	&str. These are from Chapter 4, if you need a refresher. The String type,
	however, is not the same as the str literals that the compiler will bake
	into your binaries. The String type objects are growable (like vectors),
	mutable, owned and UTF-8 encoded. (str is also UTF-8.)

Creating a New String:

	Many of the features provided in the implementation of vectors are also
	available for the String type, as String is implemented as a wrapper around
	a vector of bytes. So, making a new String is the same (as is making a new
	instance of most things):*/

	let mut string = String::new();

	// We can also call to_string on any method implementing the Display trait:

	let data = "initial contents"; // type of 'data' is str

	let s = data.to_string(); // type of 's' is now String

	let s = "initial contents".to_string(); // type of s is also String here

	let s = String::from("initial contents") 	// works on string literals;
												// we've seen this many times
												// in the book already.

	// Strings are UTF-8 encoded, so we can include any glyph:

	let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");		// all valid strings!

    /*

Updating a String:

	As Strings are just specialized wrappers around vectors, working with them
	is the same in a few ways. Strings can grow and change contents, and we
	can use the + operator or format! to concatenate Strings.

Appending to a String with push_str and push:

	We can grow a String using the push_str method to append a string slice:*/

	let mut s = String::from("foo");
	s.push_str("bar");

	/*

	This leaves s as having contents "foobar". Note that push_str takes a &str
	so that we do not take ownership of that slice and the slice remains valid
	after being pushed to the string. Here is an example of valid code using
	this property:*/

	let mut s1 = String::from("foo");
	let s2 = "bar";
	s1.push_str(s2);		// &str passed here, so s2 is not consumed
	println!("s2 is {s2}"); // s2 was not consumed, so println! call is legal.

	// In contrast to push_str(), push() appends a single character:

	let mut s = String::from("lo");
	s.push('l'); 		// s now contains "lol"

	/*

Concatenation with the + Operator or the format! Macro:

	Concatenating strings can be useful from time to time. Maybe just a little.
	We can conveniently do so with a +:*/

	let s1 = String::from("Hello, ");
	let s2 = String::from("World!");
	let s3 = s1 + &s2; // Note s1 is consumed here, but s2 is not because of &.

	/*

	I thought this mixture of reference and value was a stylistic choice by The
	Book to remind us of ownership rules, but it is not. The implementation of
	+ looks like:*/

	fn add(self, s: &str) -> String {

	/*

	So we necessarily have to use a reference to our second String, because the
	signature for add specifies an &str. But wait, &s2 is an &String, not an
	&str. If our type doesn't match the function signature, then why does it
	compile?

	The compiler can coerce the &String into an &str. When we call add, Rust
	applies deref coercion, which turns &s2 into &s2[..]. Recall that this
	syntax means we are taking a string slice of the entirety of s2, which is
	in fact of type &str. &str types do not consume the strings they come from
	when used, so s2 remains valid. Deref coercion is discussed more in Chapter
	15.

	Also note that the first parameter, self, is not a reference. It takes
	ownership, consuming the first parameter. So, in our example, s1 is moved
	into s3 and the content of s2 is copied and concatenated onto s3.

	If we want to add a lot of strings, + can be difficult to read. Ex:*/

	let s1 = String::from("tic");
	let s2 = String::from("tac");
	let s3 = String::from("toe");

	let s = s1 + "-" + &s2 + "-" + &s3; // s = "tic-tac-toe"

	// We can use format! instead for clarity:

	let s = format!("{s1}-{s2}-{s3}"); // much easier to read

	/*

	Note that format! works like println! except it returns a string with the
	contents. It also doesn't take ownership of any of the parameters passed
	to it.

Indexing into Strings:
	
	Even though Strings are wrappers around vectors, we cannot index into them
	the same way that we can with vectors. Examine this code and the error it
	produces during compilation:*/

	let s1 = String::from("hello");
	let h = s1[0];

	/*

	error[E0277]: the type `String` cannot be indexed by `{integer}`
	 --> src/main.rs:3:13
	  |
	3 |     let h = s1[0];
	  |             ^^^^^ `String` cannot be indexed by `{integer}`
	  |
	  = help: the trait `Index<{integer}>` is not implemented for `String`
	  = help: the following other types implement trait `Index<Idx>`:
	            <String as Index<RangeFrom<usize>>>
	            <String as Index<RangeFull>>
	            <String as Index<RangeInclusive<usize>>>
	            <String as Index<RangeTo<usize>>>
	            <String as Index<RangeToInclusive<usize>>>
	            <String as Index<std::ops::Range<usize>>>
	            <str as Index<I>>

	For more information about this error, try `rustc --explain E0277`.
	error: could not compile `collections` due to previous error

	Reading this error, there are two things I can take away immediately: for
	some reason, indexing by integer is not allowed. The Book stated earlier
	that Strings are collections of the byte representations of their char
	literals, so UTF-8 may be the cause of that. Similarly, the compiler says
	we can use a Range of a variety of subtypes to index the string - including
	a range of s[0..1], which would be the equivalent of s[0]. So, there is
	probably some conversion between the characters in the string and the bytes
	they represent going on either when the string is instantiated or updated,
	or when the data is accessed. Let's see what the book has to say about it:

Internal Representation:

	A String is a wrapper over a Vec<u8>. Let's consider some strings:*/

	let hello = String::from("Hola");

	/*

	Here, len is 4, which means the vector is 4 bytes long. Each letter takes
	1 byte with UTF-8 encoding. Now consider this string:*/

	let hello = String::from("Здравствуйте");

	/*

	This appears to be 12 characters long, so upon cursory inspection, it looks
	like it should have a len of 12. But consider what one byte means: you can
	store 256 unique values in 1 byte, or 256 unique characters. ASCII alone
	takes up half of that space. UTF-8 might not have the same encodings as
	ASCII, but the point is that ASCII just goes through English alphanumerics
	and keyboard control sequences. There are a lot of languages and many of
	them have a ton of unique characters, so there is no way 1 byte is enough
	to encode all of them.

	In the above string, each character actually takes up two bytes of space.
	Hence, indexing the string will not return the information you think it
	does when you request the item at an index in that string. As a conseqeunce
	of the language's decision to encode strings as UTF-8, it does not let you
	index strings.

Bytes, Scalar Values, and Grapheme Clusters:

	UTF-8 gives us several equally-valid ways of examining strings. Let's look
	at the word नमस्ते.

	Stored as a vector of u8s, नमस्ते is represented as:

	[224, 164, 168, 224, 164, 174, 224, 164, 184, 
	 224, 165, 141, 224, 164, 164, 224, 165, 135]

	Stored as Unicode scalar values, which is Rust's char type:

	['न', 'म', 'स', '्', 'त', 'े']

	The fourth and sixth entries here are not characters themselves but instead
	they are diacritics used in the construction of the word. 

	Finally, we can view this word as grapheme clusters, which is as close as
	we can get to a general concept of a "letter":

	["न", "म", "स्", "ते"]

Slicing Strings:

	Indexing into a string doesn't work because Rust can't figure out what type
	you're asking it for, basically. But, if you provide a range, Rust will
	try to give you a group of bytes. We can do something like this:*/

	let hello = "Здравствуйте";

	let s = &hello[0..4];

	/*

	This provides us the first four _bytes_ of this string. The program will
	panic if you attempt to start or end your slice in the middle of the bytes
	of any character.

Methods for Iterating Over Strings:

	The recommended way to work on strings is to be explicit with what you want
	from the string using one of several string methods. The methods we have
	access to are .chars() and .bytes(). Graphemes, being combinations of
	characters which follow special rules, are not provided by the standard
	library. We can use these methods as follows:*/

	for ch in "Зд".chars() {
		println!("{ch}");		//	prints the characters one at a time
	}

	for byte in "Зд".bytes() {
		println!("{byte}");		//	prints the bytes that make up each char,
	}							//	one at a time. You will get 4 lines, here.

	/*

	When working with bytes, remember that Unicode values may be made of more
	than 1 byte. This means that if you are trying to separate characters out
	into different variables, bytewise, you might incorrectly split your string
	if it contained characters with a length of more than 1 byte.

Strings Are Not So Simple:

	Strings are complicated! This section took more lines than the vector
	section, which is actually incredible. The Book says that string handling
	is a part of language design that is undertaken with intentionality, and
	Rust has chosen to force all String data to be handled properly. This means
	things are more difficult up front, but saves errors down the line. It is
	annoying as an individual programmer trying to learn through small
	programs, but considering a mistake that propagates through a stage of the
	development process takes ten times longer to fix if not caught in time, it
	seems like a reasonable tradeoff for a language with production code in
	mind.

Storing Keys with Associatd Values in Hash Maps:

	The last collection we will talk about in detail here is the Hash Map. This
	collection is common everywhere, and extremely valuable. In Rust, it is
	generalized as a HashMap<K, V>. This mapping of key-to-value is called a
	hashing function.

Creating a New Hash Map:

	Like other structures in Rust, we can just call new() and add elements to
	it. The simplest function for adding elements to a map is insert(), which
	is distinct from the vector's push(). We can declare a map like so:*/

	use std::collections::HashMap;

	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Green"), 15);

	/*

	Note that we have to explicitly bring HashMap into scope before we can use
	it. Maps are useful, but not quite as ubiquitous as vectors or strings, so
	they are not included with the prelude.

	Like vectors, maps are heap-allocated, and need to know the types they are
	associated with in order to request the proper amount of memory. They are
	also homogeneous, which means all keys are of the same type, and all values
	are of the same type (keys and values can be 2 different types).

Accessing Values in a Hash Map:

	We grab values from a hash map using get():*/

	use std::collections::HashMap;

	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Blellow"), 95);

	let team_name = String::from("Blellow");
	let score = scores.get(&team_name).copied().unwrap_or(0); //wtf, this line?

	/*

	This looks pretty standard for maps until the very end. Let's break down
	what is happening in this line. get() returns an Option<&V>. Remember, this
	means the two possible values are Some(&V), and None. So, get() returns
	an option holding a reference. It is None if the key does not exist. If we 
	want to use this elsewhere, we need to copy the value out of the map.
	copied() turns our Option<&V> into an Option<i32>, because our values here
	are i32s. This i32 can then be bound to some other variable; here, that
	variable is score. Finally, unwrap_or() is a map-unique method that sets
	that key's value to its input if no value is found for that key. 

	We can iterate through elements of a map using a for loop. When we do so,
	the map will give us key - value pairs as tuples, and we need to handle
	them both at once:*/

	for (key, value) in &scores {
		println!("{key}:, {value}");
	}

	/*

	Because a map does not enforce any sort of ordinality, the key-value pairs
	are processed in an arbitrary order here.

Hash Maps and Ownership:

	For types implementing the Copy trait, values are copied into the map when
	inserted. For owned values, the map takes ownership once the value is
	inserted. For example:*/

	use std::collections::HashMap;

	let field_name = String::from("Favorite Color");
	let field_value = String::from("Scarlet");

	let mut map = HashMap::new();

	map.insert(field_name, field_value);	// field_name and field_value have
											// now been consumed, and are not
	/*										// valid for referencing.
	
	The map takes ownership of field_name and field_value here. If we want to
	avoid that transfer of ownership, we can provide the map with references
	instead, but it becomes more complicated. Since the value pointed to by
	the reference within the map could be dropped before the map, we have to
	make guarantees to the compiler that that won't happen. The compiler would
	be very unhappy otherwise, because trying to reference dropped values is a
	memory management error and basically what this entire language was made to
	avoid in the first place. We make these guarantees with lifetimes, and they
	are covered in Chapter 10.

Updating a Hash Map:

	Hash maps can hold an arbitrary number of key-value pairs. Note that for
	each key, only one value can be associated with it. If we have a conflict,
	then, we have to figure out how to resolve that conflict. There are a few
	options. We can:

		-	Overwrite the value:

			If we insert a value into a key that already exists, the existing
			value will be overwritten and lost. Sometimes, that's fine! ex:*/

			use std::collections::HashMap;

			let mut map = HashMap::new();

			map.insert(String::from("key"), String::from("first_value"));

			map.insert(String::from("key"), String::from("overwriting_value"));

			/*

			map.get("key") would return Some(&"overwriting_value").

		-	Add the key and value only if the key does not already exist:

			If we want to avoid overwriting values, we can do this instead of
			the above. We use a method called entry() for this. The return
			value of entry() is a variant of enum Entry. Let's look at what
			using this pattern looks like:*/

			use std::collections::HashMap;

			let mut map = HashMap::new();

			map.insert(String::from("Blue"), 10);

			map.entry(String::from("Yellow")).or_insert(50);
			map.entry(String::from("Blue")).or_insert(50);

			println!("{:?}", map);

			/*

			We would end up printing {"Yellow": 50, "Blue": 10} with this code.
			The lines containing entry() are saying: if entry(key) exists, do
			nothing. If it does not, add that key associated with the value
			passed to or_insert(value). It then returns a mutable reference to
			the new value.

		-	Update a value based on the old value:

			Sometimes, we care about keeping track of logical relationships
			that change over time. When these things change, we want to update
			to a new value that is relative to the one we already have. Some
			simple cases include counting something, or appending values to
			some growable container (I am thinking strings). The Book provides
			a code snippet that counts how many times each unique word appears
			in a selection of text, which displays updating functionality:*/

			use std::collections::HashMap;

			let text = "hello world wonderful world";

			let mut word_counts = HashMap::new();

			for word in text.split_whitespace() {	// gets iterator over words
				let count = map.entry(word).or_insert(0);
				*count += 1;	// Note dereferencing the obtained value, just
			}					// as we had to do for this behavior in vectors

			println!("{:?}", map);

			/*

			This prints {"world": 2, "hello": 1, "wonderful": 1}. The order
			might be different, as we stated earlier it is arbitrary. Note the
			split_whitespace method on the string; this returns an iterator
			over the slices of the string corresponding to words stripped of
			whitespace. The or_insert method returns &mut V to the value for
			the key. This means we hold a mutable reference to the value after
			this line, regardless of whether the key existed or not; if it did
			not exist, it is simply initialized to the specified value and we
			continue on as normal. Because we're holding a mutable reference,
			we need to dereference the pointer to change the value it points
			to, hence *count += 1;. Once the control statement ends, the
			mutable reference is dropped, so we can continue to use immutable
			references to this data.

Hashing Functions:

	HashMap has a default hashing function called SipHash that is resistant to
	DoS attacks that target hash tables. This hash trades some efficiency for
	better security; if this is not suitable for our use case, we can implement
	our own hashing function by creating a type implementing the BuildHasher
	trait. Or find one on crates.io.

Conclusion:

	The Book has some example problems to be solved:

	Given a list of integers, use a vector and return the median (when sorted, 
	the value in the middle position) and mode (the value that occurs most 
	often; a hash map will be helpful here) of the list.


	Convert strings to pig latin. The first consonant of each word is moved to 
	the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words 
	that start with a vowel have “hay” added to the end instead (“apple” 
	becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!


	Using a hash map and vectors, create a text interface to allow a user to
	add employee names to a department in a company. For example, “Add Sally to 
	Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of 
	all people in a department or all people in the company by department, 
	sorted alphabetically.

	It also suggests to browse the documentation for these containers to find
	methods which may be helpful for solving these problems. It will be done!
	