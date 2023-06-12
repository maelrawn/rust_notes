/*

These are my notes on Chapter 5 of The Book.

Defining and Instantiating Structs:

	Structs fill a similar role to tuples, but both have a greater degree of
	structure to the information they contain, and are more flexible, in a way.
	Because we name the elements of a struct, we can reference those elements
	by name instead of by their position within the struct, like we have to do
	with tuples. This removes the manual bookkeeping inherent to tuples and
	instead allows us to use the data in structs much more comfortably.

	We define a struct with the struct keyword, and then define its internals
	with a name and a type, like so:*/

	struct User{
		active: bool,
		username: String,
		email: String,
		sign_in_count: u64, //Note the convenient trailing comma!
	}

	/*

	Now that our struct is defined, we can instantiate it by providing values
	for each of the fields as key:value pairs:*/

	let user = User{
		active: true,
		username: String::from("username"),
		email: String::from("user@name.com"),
		sign_in_count: 1,
	};	//Note the trailing semicolon!

	//When we want to access an element, we use dot notation. Also note an
	//instance of a struct declared mutable makes all its elements mutable.

	let mut user = User{ ... };

	user.email = String::from("new@email.com");

	//We can also write functions to initialize structs for us, because the
	//aforementioned method can be pretty cumbersome if some of our fields can
	//be initialized to default values:

	fn build_user(email: String, username: String) -> User {
		User {
			active: true,
			email: email,
			username: username,
			sign_in_count: 1,
		}		//Note the lack of semicolon indicating this User is returned
	}

	//We can also simplify this definition a bit by using shorthand. If a
	//parameter and struct field have the same name, then we can just specify
	//the name and Rust will know we're trying to assign that parameter to that
	//field. Our function then becomes:

	fn build_user(email: String, username: String) -> User {
		User {
			active: true,
			username,		//shorthand because of match in param list
			email,			//shorthand because of match in param list
			sign_in_count: 1,
		}
	}

/*

Struct Update Syntax:

	We frequently need to make new structs from old structs, or referencing
	their data. If we needed to make a new user, for example, we could do it
	like this:*/

	let user1 = User { ... };
	let user2 = User {
		active = user1.active,
		email = user1.email,
		username = user1.username,
		sign_in_count = 2,
	};

	//Or we can use update syntax to only specify the things we've changed:


	let user1 = User { ... };
	let user2 = User {
		sign_in_count = 2,
		..user1 //This copies all fields from user1 that were not specified
	};

	/*

	This does use move assignment! user1 is no longer valid after line 90,
	because String is heap-allocated and thus ownership has been transferred
	to user2. If we had given user2 new values for the heap-allocated types,
	then user1 would not have moved anything and still be valid.

Tuple Structs:

	These are fast and dirty ways to define structs for a few closely related
	pieces of data. They are declared with the struct keyword, a chosen name,
	and the types desired. Some examples:*/

	struct Color(i32, i32, i32);	//these variables are unnamed,
	struct Point(i32, i32, i32);	//but the tuple type is named.

	fn main() {
		let black = Color(0, 0, 0); //we access these values the same way
		let origin = Point(0, 0, 0);//we access values in any other tuple.
	}

	/*

Unit-Like Structs Without Fields:

	When we talked about tuples, we defined (), the unit type. We can define
	unit-like structs for custom types, too. The Book says these are useful for
	when we want to implement a trait on a type, but don't have any data to
	store within that type. The point here is that we can define methods for
	this type to create functionality for a trait, which can then be inherited
	by other types. 

	Unit-like structs are just structs without members:*/

	struct AlwaysEqual;

	fn main() {
		let subject = AlwaysEqual; //weird, I know. We're missing some context.
	}

	/*

	Note that, as we have talked about them thus far, structs must own their
	data. This means that we could not store a reference to a heap-allocated
	object in a struct. The reason is that if we did store a reference in a
	struct, the reference could expire, rendering our struct invalid. The
	solution to this is to use lifetimes to guarantee the reference lives as
	long as the struct does, which is covered in Chapter 10. 

	Before reading the section on lifetimes, I'm going to go ahead and guess 
	that adding lifetimes annotation to a variable turns its individually-owned 
	pointer into a shared pointer, as in the c++ smart pointer type. This would 
	mean that a value annotated with a lifetime doesn't get dropped until every
	owner of a lifetime-annotated reference to that value is dropped.

Example Program Using Structs:

	The following code is runnable in a project in the rectangles/ directory.

	We're going to make a program that calculates the area of a rectangle.

	We can start by doing it in the most naive way possible, with ungrouped
	variables:*/

	fn main() {
		let width1 = 30;
		let height1 = 50;

		println!(
			"The area of the rectangle is {} square pixels.", 
			area(width1, height1)
		);
	}

	fn area(width: u32, height: u32) -> u32 {
		width * height
	}

	/*

	This works! It does output what we wanted. But if we wanted to make this
	part of a larger program (and let's be real, we can calculate the area of
	rectangles in our sleep, so it has to be part of something larger to be
	useful at any point) then we would want these things to be clearly
	logically connected, instead of floating around as free variables. Also,
	different shapes have different area calculations, some also using only a
	width and height, so area's signature does not actually tell us anything
	about what it's calculating!

	So, let's go back and refactor this with tuples:*/

	fn main() {
		let rect1 = (30, 50);

		println!(
			"The area of the rectangle is {} square pixels",
			area(rect1)
		);
	}

	fn area(dimensions: (u32, u32)) -> u32 {
		dimensions.0 * dimensions.1
	}

	/*

	This is better, but there are still some problems. We still haven't made
	clear what area we are calculating, and now instead of being able to read
	the function to maybe get a guess at it, we have to rely on the name of
	the variable that we're passing in to maybe tell us what we're getting!
	Additionally, if we wanted to do anything involving the width and height
	of our rectangle, we would have to magically know that the width is the
	first element of the tuple and the height is the second. 

	Good code is self-documenting. This code is not! So, let's use a struct to
	introduce meaning into our code.*/

	struct Rectangle {
		width: u32,
		height: u32,
	}

	fn main() {
		let rect1 = Rectangle {
			width: 30,
			height: 50,
		};

		println!(
			"The area of the rectangle is {} square pixels", 
			area(&rect1)
		);
	}

	fn area(rectangle: &Rectangle) -> u32 {
		rectangle.width * rectangle.height
	}

	/*

	So, our code now explains clearly to the reader what the intent is! We have
	some good code here. Also note that in our area function, we ask for a 
	reference to the rectangle instead of the rectangle itself. We don't want
	to take ownership of the struct, so that main can continue to use it.
	Note that accessing elements of a borrowed variable does not consume those
	variables if they are used; they are effectively also borrowed.

Adding Functionality with Derived Traits:

	If something was wrong with our rectangle, it might be nice to be able to
	print out all its elements easily. However, if we try this:*/

	println!("rect1 elements: {}", rect1);

	/*

	We get a compilation error:

	error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
	= help: the trait `std::fmt::Display` is not implemented for `Rectangle`
	= note: in format strings you may be able to use `{:?}` (or {:#?} 
		for pretty-print) instead


	The println! macro uses a type's implementation of the std::fmt::Display
	trait in order to do its thing. Because we defined Rectangle on our own,
	it doesn't implement Display yet, as we didn't implement it ourselves! We
	can try to do what the compiler suggests in the above error message:*/

	println!("rect elements: {:?}", rect1);

	/*

	But this still gives an error:

	error[E0277]: `Rectangle` doesn't implement `Debug`

	Debug is another trait that we haven't implemented. The compiler also says:

	= help: the trait `Debug` is not implemented for `Rectangle`
	= note: add `#[derive(Debug)]` to `Rectangle` or manually 
		`impl Debug for Rectangle`

	This is a derived trait. For some reason, derived traits are not explained
	here and instead just used. However, I think it has to be a trait that can
	implement itself for a given type, if the type is given the trait. The
	output of println! once Rectangle is given the derived Debug trait might
	offer some explanation:*/

	#[derive(Debug)]
	struct Rectangle {
		width: u32,
		height: u32
	}

	fn main() {
		let rect1 = Rectangle {
			width: 30,
			height: 50,
		};

		println!("rect1 elements: {:?}", rect1);
	}

	/*

	This code produces this output: 
	
		"rect1 elements: Rectangle { width: 30, height: 50 }"

	which does what we wanted! Note that my guess earlier about derived traits
	deriving their implementation from the type they are attached to hinges on
	this output: it's really just gone elementwise and printed the associated
	values, which seems like it would be easy to generalize.

	We could also use the dbg! macro if we add #[derive(Debug)] to a type,
	which offers a little bit more information about our object when we ask.
	An example usage of dbg! is shown below:*/

	#[derive(Debug)]
	struct Rectangle {
	    width: u32,
	    height: u32,
	}

	fn main() {
	    let scale = 2;
	    let rect1 = Rectangle {
	        width: dbg!(30 * scale), 	//dbg! takes ownership to do its thing,
	        							//but then returns the value, so we can
	        							//use it to check values during binding
	        							//like this.
	        height: 50,
	    };

	    dbg!(&rect1);	//dbg! takes ownership but we aren't reassigning rect1,
	    				//so we pass it a reference as to hold onto our rect1.
	}

	/*

	This is just one example of the flexibility afforded to us from derived
	traits. There is a full list of derived traits and their behaviors in
	Appendix C of The Book, which will be useful as further reading.

Method Syntax:

	Our Rectangle code is pretty good so far, but the area function is still
	not logically bound to the type itself, despite existing only for the sake
	of that type. We can fix this by turning area() into a method on Rectangle.

	Methods are the same as functions but must be defined for a type and always
	have &self as their first parameter. Let's change area into a method:*/

	#[derive(Debug)]
	struct Rectangle {
		width: u32,
		height: u32,
	}

	impl Rectangle {
		fn area(&self) -> u32 {
			self.width * self.height
		}
	}

	fn main() {
		let rect1 = Rectangle {
			width: 30,
			height: 50,
		};

		println!(
			"The area of this rectangle is {} square pixels.", 
			rect1.area()
		);
	}

	/*

	So what is happening here? We moved the area function into a block labeled
	impl Rectangle. This block means that everything within the block will be
	associated with the Rectangle type. 

	Note that in our definition of area(&self), the &self is short for
	self: &Self. In an impl block, Self is an alias for the type the impl block
	is for, so writing area(&self) in the impl block is really just shorthand
	for area(self: &Rectangle). We can also have methods that take ownership,
	borrow immutably (what we did here), or borrow mutably.

	Organizing our code in this way makes it extremely clear what the intended
	purpose of everything is. area(&self) would not be implemented on Rectangle
	if it did not specifically mean "I want the area of this rectangle."

	Note we can have methods with the same name as fields. This is the common
	syntax for a getter in rust. For example, if we wanted to write a getter
	for Rectangle.width, impl Rectangle {width(&self) -> u32 { self.width }}
	would do the trick. The value is still accessible by writing .width, and
	the method is accessible by writing .width(). This becomes more useful in
	Chapter 7 when we go over access modifiers.

	Note we do not have a distinction between method calling syntax based on
	pointer/nonpointer like in C (-> and .). Instead, Rust automagically puts
	the correct one into place whenever you call a method with .. This is
	called automatic referencing and dereferencing.

Methods With More Parameters:

	We can write methods that take parameters that are not &self. Let's write
	a method that checks if the calling rectangle can fit within another one.*/

	struct Rectangle {
		width: u32,
		height: u32,
	}

	impl Rectangle {
		fn can_hold(&self, other: &Rectangle) -> bool {
			(self.width > other.width && self.height > other.height)
			|| (self.width > other.height && self.height > other.width)
		}
	}	//note this definition implies possible rotation of rectangles.

	fn main() {
	    let rect1 = Rectangle {
	        width: 30,
	        height: 50,
	    };
	    let rect2 = Rectangle {
	        width: 10,
	        height: 40,
	    };
	    let rect3 = Rectangle {
	        width: 60,
	        height: 45,
	    };

	    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
	    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
	}

	/*

Associated Functions:

	When we started this section, we had an issue with code organization. We
	have some solution now, but it's only a partial solution. What if we wanted
	to have a function associated with Rectangle, but not necessarily require
	an instantiated Rectangle to call? That's where associated functions enter.

	If we place a function in an impl block but do not designate self as a
	parameter, we create an associated function. It is still in the type's
	namespace (like String::from()) but does not require an object to call it
	from. The Book says these are often for constructors, but not necessarily.
	An example given:*/

	impl Rectangle {
		fn square(size: u32) -> Self {
			Self {
				width: size,
				height: size,
			}
		}
	}

	/*

	Note the use of Self as type declaration in this associated function. As
	before, it is shorthand for the type the impl block is being declared for.
	The correct way to call square() would be Rectangle::square(x). 

	Also note we can have as many impl blocks as we like. For methods and
	associated functions, this isn't really necessary, but for traits and
	generics it can be very helpful. Each trait can have multiple methods
	associated with it, and a type may have many traits, so separating those
	blocks can bring clarity to code.

	Those are the basics for structs. It's pretty similar to structs in c, but
	adapted for the way Rust wants you to think about things, and with some
	nicer formatting options. Don't forget about the tuple structs, though;
	those seem very nice for certain applications.