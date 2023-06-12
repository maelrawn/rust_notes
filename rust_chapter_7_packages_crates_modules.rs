/*

These are my notes on Chapter 7 of The Book.

When problems become problem sets, and when problem sets become large, we need
a lot of code. Being able to organize code by file, directory, and package, and
include what we need from those chunks, is a fundamental part of any successful
project.

To that end, Rust provides several concepts: scope, packages, crates, modules,
and paths.

Packages and Crates:

	A crate is the smallest unit of Rust code considered by the compiler. Any
	file you pass into the compiler is considered a crate. Crates can contain
	modules, and modules may be defined in that crate, or in crates included
	with the parent crate.

	There are two kinds of crates: binary crates and library crates. Binary
	crates are compilable programs, which then produce executables. These must
	all have fn main() {} defined so the program has an entry point.

	Library crates lack fn main() {} and do not compile to an executable. They
	define reusable functionality intended to be used in other projects to 
	solve some kind of problem. One we have used already in The Book is rand.
	rand cannot compile into an executable program on its own, but can be
	included in another crate to provide needed functionality. 

	Generally, "crate" refers to library crates, and this is generally 1-to-1
	with the concept of a library (hence library crates). It begs the question,
	however: why does everything created after python (or java maybe?) need
	some funny thematic name scheme? For people concerned with the best way to
	communicate an idea through code, it seems counterintuitive to go and
	relabel commonly accepted ideas for a cute joke. Anyways,

	The crate root is the source file that the compiler starts its work from.
	This is also the root module of your crate.

	A package is a grouping of crates. It contains its own Cargo.toml which
	tells the compiler how to build the crates. Cargo is its own package
	containing a binary crate and a library crate, which the binary crate
	depends on. Other projects can depend on the Cargo library crate to gain
	functionality implemented by Cargo.

	A package must contain at least 1 crate, at most 1 library crate, and any
	number of binary crates.

	Whenever we run Cargo new, we're making a new package. src/main.rs is the
	binary crate root, and src/lib.rs is the library crate root. Cargo passes
	these to rustc when you want to compile something.

	If we want to create multiple binaries, then we can place files in the 
	src/bin directory. Each file in src/bin is a separate binary crate.

Defining Modules to Control Scope and Privacy:

	This is where we gain access modifiers for our code. We haven't thought
	about our code in the broader context of other pieces of code, yet, so
	we didn't need to think about this, but now we're ready.

Modules Cheat Sheet:

	The book provides a straightforwards, bulleted list of things that happens
	when we compile, how that changes when different keywords are used, and how
	many developers organize their code.

	-	Start from the create root: The compiler first looks for src/lib.rs or
		src/main.rs for code to compile.

	-	Declaring modules: In the crate root, we can define new modules. For
		example, we may declare "garden" with mod garden;. The compiler then
		looks for code for the module in these places, in order:

		-	Inline, in the braces you may provide after mod garden{};

		-	In the file src/garden.rs,

		-	In the file src/garden/mod.rs.

			-	Note we have a way to define inline, a way to define in a
				different file in the same directory, and a way to define in a
				file in a subdirectory. Those are all the tools we need to make
				expansive library structures.

	-	Declaring submodules: We can declare submodules in any file besides the
		crate root. (As mentioned above!)  We could declare mod vegetables; in
		src/garden.rs. The compiler will then look for the definition of
		vegetables in the same locations as above.

	-	Paths to code in modules: Once a module is part of your crate, we can
		refer to code in that module from anywhere else in the same crate, as
		long as the privacy rules allow, using the path to the code. If we
		wanted to reference Asparagus from the garden vegetables module, we
		would write crate::garden::vegetables::Asparagus.

	-	Private vs Public: Code within a module is private from its parent
		modules by default. So, Asparagus can access code from vegetable, but
		not vice-versa. To make a module public, declare it pub mod instead of
		mod. To make the individual items in a module public, declare them as
		pub before their declarations.

	-	The use keyword: Within a scope, the use keyword creates shortcuts to
		items to reduce repetition of long paths. This is the same concept as
		namespacing. For example, if we wanted to refer to the odious
		crate::garden::vegetable::Asparagus more than once, we could just write
		use crate::garden::vegetable::Asparagus, and from then on we would only
		need to write Asparagus to reference Asparagus.

	Here is an example of these modifiers in action. Our example project
	structure is like so:

		backyard
		├── Cargo.lock
		├── Cargo.toml
		└── src
		    ├── garden
		    │   └── vegetables.rs
		    ├── garden.rs
		    └── main.rs

		The crate root is src/main.rs, and it looks like this:*/

			use crate::garden::vegetables::Asparagus;

			pub mod garden;

			fn main() {
			    let plant = Asparagus {};
			    println!("I'm growing {:?}!", plant);
			}

		// The line pub mod garden; tells the compiler to include the code it
		// finds in src/garden.rs. That file contains only this line:

			pub mod vegetables;

		// which indicates the code in src/garden/vegetables.rs is also to be
		// included. That file contains:

			#[derive(Debug)]
			pub struct Asparagus {}

		/*

		Which means main has access to the struct Asparagus through this chain
		of public modules. Note that our directory structure could look
		different and achieve the same goals; we are given flexibility with the
		compiler rules we were given before. (Line 75 of this file).

Grouping Related Code in Modules:

	Modules let us organize code for readability, but they also allow us to
	control privacy and enforce encapsulation. The code within a module is
	private by default (implied by the pub keyword used prior).

	Here is one such example of structuring related code via modules:*/

	mod front_of_house {
	    mod hosting {
	        fn add_to_waitlist() {}

	        fn seat_at_table() {}
	    }

	    mod serving {
	        fn take_order() {}

	        fn serve_order() {}

	        fn take_payment() {}
	    }
	}
	
	/*

	Note that modules can hold definitions for anything, not just functions.
	Also note that these modules form a tree, hence the 'crate root' label:

	crate
	 └── front_of_house
	     ├── hosting
	     │   ├── add_to_waitlist
	     │   └── seat_at_table
	     └── serving
	         ├── take_order
	         ├── serve_order
	         └── take_payment

	Here, hosting and serving nest inside front_of_house, making them siblings.
	This just means that they are defined in the same module. If module A is
	defined inside of module B, we say that A is the child of B, and B is the
	parent of A.

Paths for Referring to an Item in the Module Tree:

	There are two ways to tell Rust where an item is in the module tree. These
	are pretty standard:

		-	An absolute path starts from the crate root; literally the first
			module is crate::.

		-	A relative path starts from the current module and uses self,
			super, or an identifier within the current module.

	As an example for how these should be used, consider the following:*/

	mod front_of_house {
	    mod hosting {
	        fn add_to_waitlist() {}
	    }
	}

	// Say we have a function and and want to call add_to_waitlist() from
	// inside of it. The two ways following are correct. Note that this would
	// be a function exposed in the public api, and so is marked pub.

	pub fn eat_at_restaurant() {
	    // Absolute path
	    crate::front_of_house::hosting::add_to_waitlist();

	    // Relative path
	    front_of_house::hosting::add_to_waitlist();
	}

	/*

	Choosing to use absolute or relative paths is a design decision impacted by
	how you envision the code might change in future revisions. Relative paths
	won't break if we move the whole environment from the first module of that
	relative path into some other location, but if something internal to that
	chain moves, that path breaks. Absolute paths stay valid if we are changing
	the location of the caller, not the callee.

	There is a problem with the above code snippet, however. The modules are
	currently private, so even though we have the correct paths, we are not
	allowed to invoke the specified function. All items declared in modules are
	private to their parents by default. 

	To be more clear, items in a parent module cannot access their child
	modules, but child modules can access the items in their parent modules.

Exposing Paths with the pub Keyword:

	Here's the error we get if we try to compile the prior snippet:

	$ cargo build
	   Compiling restaurant v0.1.0 (file:///projects/restaurant)
	error[E0603]: module `hosting` is private
	 --> src/lib.rs:9:28
	  |
	9 |     crate::front_of_house::hosting::add_to_waitlist();
	  |                            ^^^^^^^ private module
	  |
	note: the module `hosting` is defined here
	 --> src/lib.rs:2:5
	  |
	2 |     mod hosting {
	  |     ^^^^^^^^^^^

	error[E0603]: module `hosting` is private
	  --> src/lib.rs:12:21
	   |
	12 |     front_of_house::hosting::add_to_waitlist();
	   |                     ^^^^^^^ private module
	   |
	note: the module `hosting` is defined here
	  --> src/lib.rs:2:5
	   |
	2  |     mod hosting {
	   |     ^^^^^^^^^^^

	For more information about this error, try `rustc --explain E0603`.
	error: could not compile `restaurant` due to 2 previous errors

	From reading these errors, it might make sense to add pub to our hosting
	module. However, that doesn't fix everything outright. These are the errors
	which result if we just change hosting to pub:

	$ cargo build
	   Compiling restaurant v0.1.0 (file:///projects/restaurant)
	error[E0603]: function `add_to_waitlist` is private
	 --> src/lib.rs:9:37
	  |
	9 |     crate::front_of_house::hosting::add_to_waitlist();
	  |                                     ^^^^^^^^^^^^^^^ private function
	  |
	note: the function `add_to_waitlist` is defined here
	 --> src/lib.rs:3:9
	  |
	3 |         fn add_to_waitlist() {}
	  |         ^^^^^^^^^^^^^^^^^^^^

	error[E0603]: function `add_to_waitlist` is private
	  --> src/lib.rs:12:30
	   |
	12 |     front_of_house::hosting::add_to_waitlist();
	   |                              ^^^^^^^^^^^^^^^ private function
	   |
	note: the function `add_to_waitlist` is defined here
	  --> src/lib.rs:3:9
	   |
	3  |         fn add_to_waitlist() {}
	   |         ^^^^^^^^^^^^^^^^^^^^

	For more information about this error, try `rustc --explain E0603`.
	error: could not compile `restaurant` due to 2 previous errors

	Why is this happening? Declaring pub mod hosting gives us access to hosting
	if we have access to front_of_house, but does not give us access to the
	internals of hosting.

	This is because a module is effectively a container with access modifiers.
	In order to gain access to any of its internals, that module needs to also
	explicitly expose those internals by declaring them pub as well. So, our
	module structure from earlier should be changed like this:*/

	mod front_of_house {
		pub mod hosting {
			pub fn add_to_waitlist(){}
		}
	}

	pub fn eat_at_restaurant() {
	    // Absolute path
	    crate::front_of_house::hosting::add_to_waitlist();

	    // Relative path
	    front_of_house::hosting::add_to_waitlist();
	}

	/*

	if we want our other code to be able to call add_to_waitlist(). Now let's
	break down why this all works, because it's kind of complicated. 

	Absolute path: Here, we start at crate, which is the module tree's root.
	The front_of_house module is defined in the crate root. It's not public,
	but because the function eat_at_restaurant is defined in the same module,
	(shorthand for this status is 'siblings') we can refer to front_of_house
	from eat_at_restaurant. Then, because we have access to front_of_house and
	hosting is marked pub, we can access hosting. And finally, because we have
	access to hosting and add_to_waitlist is marked pub, we can access
	add_to_waitlist, and our function call succeeds.

	Relative path: It's the same as the absolute path, except the first step
	differs: We start from front_of_house instead of the crate root. Because
	front_of_house and eat_at_restaurants are siblings regardless of how we
	choose to write the path, we get the same result.

	When we choose to organize code like this, note that the restrictions we
	apply internally are the same restrictions we are applying to potential
	users of the code. We are required to write APIs that meet our standards
	because we are treated as users by our own code, rather than as authors who
	are allowed to write stuff that just sort of works and then passing it off
	onto users.

Starting Relative Paths with super:

	We can construct relative paths that begin in the parent module by starting
	our path with the super keyword. This is useful for closely related modules
	that will remain closely related if code is restructured. The Book provides
	an example using a chef:*/

	fn deliver_order() {}

	mod back_of_house {
		fn fix_incorrect_order() {
			cook_order();
			super::deliver_order();
		}

		fn cook_order() {}
	}

	/*

	Here, fix_incorrect_order needs to access deliver_order, which is defined
	in a parent module. We reference this with super::, and because we are in
	a scope which is a child of a sibling of deliver_order, we are allowed to
	access deliver_order. (An easier but more ambiguous way to say "y child of 
	a sibling of x" would be to say "x is an ancestor of y").

Making Structs and Enums Public:

	If we designate a struct as pub, that does not give access to the struct's
	internals. Similarly to how the module pub rules work, pub struct gives
	access to that container, and elements of the container need to be marked
	public in order to be accessed from outside of that container.

	Because of this, and because every element in a struct must be bound to a
	value in order to instantiate the struct, we must provide a constructor for
	the struct if we expect it to be used in an external module. Otherwise,
	nobody would be able to use the struct aside from the module implementing
	said struct, or its children.

	Enums work the other way around. If we declare an enum as pub, then all of
	the variants of that enum are also public. This should make sense; the
	point of the enum in the first place is to provide the variants, so if the
	enum were public but with private variants, we've effectively provided
	nothing to the programmer while simultaneously introducing counterintuitive
	syntax to the system.

Bringing Paths into Scope with the use Keyword:
	
	All of this stuff about paths is useful for keeping our code separate and
	concise, but it's also very cumbersome, especially in projects with large
	module trees. We can gain this useful functionality while eliminating its
	cumbersome nature through the use statement. We mentioned this before, but
	writing use <path> brings that path into scope for as long as the scope
	is valid. An example for front_of_house is below:*/

	mod front_of_house {
		pub mod hosting {
			pub fn add_to_waitlist() {}
		}
	}

	use crate::front_of_house::hosting;

	pub fn eat_at_restaurant() {
		hosting::add_to_waitlist();
	}

	/*

	This is literally just specifying a path and then appending to it later.
	It should be familiar, and is very straightforwards. Note that, as written
	above, use statements are only valid for the scope they are declared in,
	for as long as that scope remains valid. If we changed our module structure
	like so:*/

	mod front_of_house {
		pub mod hosting {
			pub fn add_to_waitlist() {}
		}
	}

	use crate::front_of_house::hosting;

	mod customer {
		pub fn eat_at_restaurant() {
			hosting::add_to_waitlist(); //Fails! use statement is not valid in
										//this scope, only the outer scope.
		} 
	}

	/*

	Our code would fail to compile, as the use statement is outside of the
	scope which references the desired statement. We could either move the use
	statement into the function, or call super::hosting::add_to_waitlist() to
	solve our problem.

	The preferred way to specify functions with use is to grab the function's
	parent module, and not the function itself. The reason being, this allows
	other programmers (including the person who wrote it, in the future) to see
	the function exists in a different module and understand why the use is
	there in the first place. i.e.*/

	use crate::front_of_house::hosting;
	hosting::add_to_waitlist(); //Good!

	use crate::front_of_house::hosting::add_to_waitlist;
	add_to_waitlist(); //Bad! Does not tell us enough about the location of fn!

	/*

	However, the opposite is true when bringing structs, enums, and other items
	into scope. If we wanted to do that, we would specify the full path of the
	item and the item itself in our use statement, like so:*/

	use std::collections::HashMap;
	let hash_map = HashMap::new();	//Good!

	use std::collections;
	let hash_map = collections::HashMap::new(); //Bad! Just by convention ig

	/*

	The exception to the above is if we have two items with the same name. In
	that case, we have to specify and call them through their parents, else the
	compiler would not know which item we meant when trying to call it.

Providing New Names with the As Keyword:

	If we really don't want to specify the parents of similarly-named items, or
	we just want to give things a new name to make them confusing, we can use
	the as keyword. It's very simple:*/

	use std::fmt::Result;
	use std::io::Result; //Bad! Compiler can't differentiate if we call Result!

	use std::fmt::Result;
	use std::fmt::Result as IoResult; //Good! Compiler can read this.

	/*

	Then wherever we wanted to reference the thing we gave a new name to, we
	would just use the new name instead of its original name. Very simple. 

Re-exporting Names with pub use:

	We can make the tangle of paths easier for people calling our code to
	navigate with pub use. Basically, when we call use, the namespace we bring
	into scope is private, i.e. it will only work for our module. If we call
	pub use instead, callers of our code now have access to the namespace we
	have defined. This is called re-exporting. This is weird, so here's the
	example and explanation given by The Book:*/

	mod front_of_house {
		pub mod hosting {
			pub fn add_to_waitlist() {}
		}
	}

	pub use crate::front_of_house::hosting;

	pub fn eat_at_restaurant() {
		hosting::add_to_waitlist();
	}

	/*

	If we did not specify our use as pub, calling code would have to specify
	add_to_waitlist as restaurant::front_of_house::hosting::add_to_waitlist(),
	which is very annoying. They'd also have to do the work of trying to figure
	out the relevant path to use. However, by writing pub use, external code
	can now call restaurant::hosting::add_to_waitlist() instead. This may not
	seem like a huge leap, but good use of this principle certainly would
	produce noticeably more easily callable code.

	It also tells other programmers how you thought about the problem domain.
	The book describes two perspectives on a restaurant: someone running a
	restaurant thinks about it as front of house and back of house, but a
	customer would not separate the restaurant like that unless they were a
	resteraunteur. By calling pub use, we codify the relationships we used when
	designing the modules, and make it clearer how they should be used.

Using External Packages:

	Recall in chapter 2 we built the guessing game involving random number
	generation. In order to get that functionality, we had to go to Cargo.toml
	and import rand = "0.8.5". This makes Cargo fetch that package and any
	dependencies.

	To then use that code, we had to bring it into the scope of our program. We
	did so by writing use rand::Rng;, which brought the Rng trait into scope,
	allowing us to call functions on types with that trait implemented.

	All crates from crates.io function this way, and so does the standard
	library. The only difference is that std does not need to be imported via
	Cargo. It does, however, still need to be referenced with a path as such.

Using Nested Paths to Clean Up Large use Lists:

	If we want to bring multiple items from a given package into scope, we can
	use a special syntax to do it all at once. Here's an example:*/

	use std::cmp::Ordering;
	use std::io;	//Non-nested imports

	use std::{cmp::Ordering, io}; //Nested imports

	/*

	The difference between these two is pretty clear. In the second example, we
	are just factoring out the common module tree path for each of these items.
	We can also use the keyword self to specify you are factoring out the whole
	path for one of your items:*/

	use std::io;
	use std::io::Write; //Non-nested

	use std::io::{self, Write}; //Nested imports using self. This will grab
								//std::io and std::io::Write for us.

	/*

The Glob Operator, or just THE GLOB:

	If we want to pull all public items from a path into scope, we can specify
	that with the * operator. This is the same as its use in specifying unix
	system paths, or similar applications. If we wanted to import all public
	items from std::collections, we would write:*/

	use std::collections::*; //This is a lot of stuff!

	/*

	The Book says that glob is often used with the tests module. I haven't
	looked at this module yet (Chapter 11) but I would imagine if you want to 
	run one of the tests, you would want to run all of the tests. Testing 
	exhaustively is much better than not.

Separating Modules into Different Files:

	Modules can get large. Many useful modules have or will have many functions
	or complicated code or both. For this reason we probably want to separate
	modules into different files. We can do so using the rules defined on line
	75. For a refresher, when a module is defined in code, the compiler looks
	for its definition in these 3 places, in sequence:

		-	Inline,

		-	In a file named <modulename>.rs in the directory of the .rs file
			which declared the module,

		-	In a file named <modulename>.rs in a subdirectory of the directory
			of the .rs file which declared the module.

	Using these rules, we can split up our restaurant code as follows:*/

	//src/main.rs internals:
	mod front_of_house;

	pub use crate::front_of_house::hosting;

	pub fn eat_at_restaurant(){
		hosting::add_to_waitlist();
	}

	/*

	The compiler sees that front_of_house is not declared inline, so it then
	looks for the file src/front_of_house.rs. This file would contain the
	internals of front_of_house's previous inline definitions, like so:*/

	//src/front_of_house.rs internals:
	pub mod hosting {
		pub fn add_to_waitlist() {}
	}

	/*

	We only need to load a file using mod once. Other files trying to reference
	that module should use the path of the file where it was originally
	declared. This means we have to traverse the module tree to the declaration
	of the module we want to use, if we want to use it, NOT traverse to another
	file which uses that module like we can with c++ include statements.

	Note that if we wanted to break this code down further, we could change
	front_of_house.rs and create a new file in src/front_of_house/hosting.rs:*/

	//src/front_of_house.rs:
	pub mod hosting;

	//src/front_of_house/hosting.rs:
	pub fn add_to_waitlist() {}

	/*

	This is kind of annoying as an example, but the point is that we can create
	as many layers of organization as we need. This also keeps the module tree
	the same, which gives us good code organization without introducing 
	unneccesary complexity. 

	That's it for this chapter. It was a very long and tedious one, all about
	bookkeeping. Bookkeeping is important, though, and we have some very nice
	tools available here. The fact that mod implies a filestructure is actually
	very nice, and both pub and use make sense how they are intended to.