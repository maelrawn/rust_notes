/*

These are my notes on Chapter 6 of The Book.

Defining an Enum:

	When we talked about structs, we used the idea of a struct to relate to one
	logical grouping of pieces of data. Our Rectangle struct is not useful if
	we have other shapes to describe. This doesn't mean structs are bad; it
	means that we can introduce another idea related to them.

	Enums are variables which may be one of many states, but only one state is
	valid at a time. So if we wanted to describe one of a variety of shapes,
	our enum could have states for Rectangle, Circle, Triangle, etc. The Book
	also gives an example of IPv4 vs IPv6 addresses: If we need to communicate
	via IP, it may be one of IPv4 or IPv6 but not both. So in code, we could
	create an enum to discretize these possibilities and handle each case
	appropriately (IPv4 and IPv6 need to be processed differently sometimes).

	This example enum might look like:*/

	enum IpAddrKind {
		V4,
		V6,
	}

	/*

	and now IpAddrKind is a custom data type available for us to use.

Enum Values:

	If we want to assign values from our enum, we need to specify the namespace
	we are referencing:*/

	let four = IpAddrKind::V4;
	let six = IpAddrKind::V6;

	//We can also write functions that take an enum type:

	fn route(ip_kind: IpAddrKind) {}

	//And then either variant would be acceptable parameters for this function:

	route(four);
	route(six);		//Both work because both are an IpAddrKind!

	/*

	The Book says it might seem natural to then define a struct that has a 
	field of type IpAddrKind to keep track of what type of IP we have, and yes,
	that would be my first thought. The combination of enum and struct might
	look something like:*/

	enum IpAddrKind {
		V4,
		V6
	}

	struct IpAddr {
		kind: IpAddrKind,
		address: String,
	}

	/*

	But there is a better way: We can associate enum types with values directly
	instead of having an intermediate struct. Above, we associated the struct
	with an IpAddrKind and a String, but we can redefine IpAddrKind to hold
	onto the String itself:*/

	enum IpAddr {
		V4(String),
		V6(String),
	}

	let home = IpAddr::V4(String::from("127.0.0.1"));

	let loopback = IpAddr::V6(String::from("::1"));

	/*

	So now each instance of the enum holds a value, and it really has purpose.
	We're taking this data and annotating it with meaningful information about
	what it represents. Note that each value in the enum is essentially a
	constructor for an instance of that value in the enum.

	Now, here's the kicker: Because these are each their own constructor, and
	each value in the enum is effectively its own subtype of the enum, they
	do not need to share parameter types. IPv4 and IPv6 are defined differently
	so we can define them differently in our enum:*/

	enum IpAddr {
		V4(u8, u8, u8, u8),
		V6(String),
	}

	let home = IpAddr::V4(127,0,0,1);

	let loopback = IpAddr::V6(String::from("::1"));

	/*

	This is pretty good. We have a definition for each value in our enum that
	is as concise as it can be while still logically grouping the V4 and V6
	addresses together under the umbrella of IpAddr.

	The Book goes on to illustrate that the standard library version of this
	same idea (IPv4 vs IPv6 processing) is extremely similar. Instead of just
	having simple tuple-structs as the enum values, however, it has structs.

	I'm going to go out on a limb and say it's for the purpose of implementing
	traits differently on each of those types. This should feel natural: each
	of IPv4 and IPv6 hold different underlying types (u8s vs String), so if
	you called consume_ip_addr(IpAddr::V4) and consume_ip_addr(IpAddr::V6),
	the function would either:

		a) need to know the internals of each type, V4 and V6, and handle them
			differently, or

		b) call traits implemented on those types and rely on them being
			correctly implemented such that they can be handled similarly,
			eliminating the need for the function to know how they work inside.

	We can make enums quite complicated, as shown below:*/

	enum Message {
		Quit,
		Move { x: i32, y: i32 },
		Write(String),
		ChangeColor(i32, i32, i32),
	}

	/*

	Note that we have several different types here: We have a unit struct in
	Quit, a real-deal whole struct in Move, a single-value tuple in Write, and
	a tuple struct in ChangeColor (technically the last two are the same type).
	The difference between these enum types and individually-defined structs
	is that these are all grouped under the Message type and can be passed
	around as such, as well as have functions implemented upon them as a group
	instead of requiring individual implementations for what may be identical
	functions (duplicate code!). If we were to implement a method on Message,
	it might look like this:*/

	impl Message {
		fn call(&self) {
			//do stuff
		}
	}

	let m = Message::Write(String::from("hello"));

	m.call(); //Example of calling a method on enum type!

	/*

The Option Enum and Its Advantages Over Null Values, or How I Learned to Stop
Worrying And Love The Enumeration:

	First, let's establish a motivation for why we use Option. Rust does not
	have a null type. Languages with a null type do it so that the programmer
	can handle the cases where a value is something or a value is nothing and
	have the compiler check it for correctness; however, most compiled
	languages with a null type cannot check for errors caused by the
	introduction of a null to a system. Nulls need to be handled correctly, and
	to do so you need to know where they can come from and where they can end
	up, which is a lot of bookkeeping. So, nulls can often introduce errors.
	Option is Rust's attempt at eliminating this error-prone pattern.

	Option is an enum that can either be something or be nothing. This is the
	same concept as a null, but more strict in its implementation. The enum
	looks like this, internally:*/

	enum Option<T> {
		None,
		Some(T),
	}

	/*

	Now that we've talked about enums, we can read this one. None is a unit
	struct which holds no data. Some(T) is a tuple struct that holds a value
	of some generic type. So, like we said, Option<T> can be either nothing,
	or something of type T.

	So, why is this useful? What we've done is effectively placed null into
	a wrapper type. This wrapper lets the compiler know where null may or may
	not be introduced, and in turn the compiler can force us to handle both
	cases for the sake of code safety. This eliminates the issue with null
	illustrated above.

	Generally, the way we use this is by matching an Option against its None
	and Some types, and handling those differently. Some will give up its
	held value, allowing you to progress the program as you expect, and None
	will allow you to handle the null or panic.

The match Control Flow Construct:

	match is like a switch statement, but more powerful. Because we can match
	on traits, names, values, wildcards, and more, we get a lot of flexibility.
	Let's write a coin sorter using match. First we'll define the coins, and
	then we'll handle the different coins differently using match:*/

	enum Coin { //Note Coin instead of Coins
		Penny,
		Nickel,
		Dime,
		Quarter,
	}

	fn value_in_cents(coin: Coin) -> u32 {
		match coin {
			Coin::Penny => 1,
			Coin::Nickel => 5,
			Coin::Dime => 10,
			Coin::Quarter => 25,
		}
	}	//Note lack of semicolon indicating we return the value of our match.

	/*

	So what's happening here? We pass in a Coin value to value_in_cents, and
	then we perform a match on that coin. Notice we can match on the type here,
	instead of if-elsing everything. This is a significant difference between
	the two structures: if statements require a boolean, but match can match
	on many patterns.

	We also have many arms to our match block. The arms may contain only one
	expression, or many lines of code, but they need to end by returning
	something (i.e. the final line of the arm must be an expression).

Patterns That Bind to Values:

	match arms can also bind to the parts of the values that match the pattern.
	This is hard to understand on first read; The Book says it is what allows
	us to extract the values from enum variants (like getting the value from
	Some in our Option variant). The Book gives an example of this:

	From 1999 to 2008, the US minted quarters with different designs for each
	state. No other coins have ever had these designs, so we can update our
	prior enum to reflect this additional data:*/

	#[derive(Debug)]
	enum UsState{
		Alabama,
		Alaska,
		//...
	}

	enum Coin {
		Penny,
		Nickel,
		Dime,
		Quarter(UsState),
	}

	/*

	If we had a collection of coins and wanted to see if we hit every state
	along the way while we counted them, we could set up our match block to
	extract and use the value associated with every Quarter:*/

	fn value_in_cents(coin: Coin) -> u32 {
		match coin {
			Coin::Penny => 1,
			Coin::Nickel => 5,
			Coin::Dime => 10,
			Coin::Quarter(state) => {
				println!("State quarter from {:?}", state);
				25
			}
		}
	}

	/*

	The way we've written the arm with the Quarter gives a name to the value
	held by Quarter so that we can use it. Remember, in the enum, the value
	held by Quarter does not have a name, so we need some way to reference it!
	Let's try doing something similar for Option<T>.

Matching with Option<T>:

	We can do with Option<T> basically what we've done above; we can write an
	arm for the case where the Option<T> is Some(T), and an arm for the case
	where Option<T> is None. We're given an example of what this might look
	like for a function that adds 1 to an i32:*/

	fn plus_one(x: Option<i32>) -> Option<i32> {
		match x {
			None => None,
			Some(i) => Some(i + 1), //note using i instead of x as x is bound
		}
	}

	let five = Some(5);
	let six = plus_one(five);
	let none = plus_one(None);

	/*

	This code works the same as the code for Coin::Quarter(UsState) but with a
	different, ubiquitous enum. This pattern is extremely common, so prepare
	to use it frequently.

Matches Are Exhaustive:

	Note that part of match's utility is that it requires the user to account
	for all cases. Remember, we're trying to write safe code, and can't allow
	undefined behavior. If we removed the None => None, arm from the previous
	block of code, the compiler would tell us we need to handle all possible
	match cases.

Catch-all Patterns and the _ Placeholder:

	Naturally, there are scenarios where you don't want to go through every
	variant in an enum and handle them differently. If we had to write an arm
	for every variant, even if they would be handled identically, match would
	suck pretty badly. So, enter the catch-all. Look at the following code:*/

	let dice_roll = 9;
	match dice_roll {
		3 => add_fancy_hat(),
		7 => remove_fancy_hat(),
		other => move_player(other),
	}

	fn add_fancy_hat() {}
	fn remove_fancy_hat() {}
	fn move_player(num_spaces: u8) {}

	/*

	Here, the pattern other functions as a catch-all. We have not enumerated
	all the possible values a u8 can have, but we have provided a variable for
	any u8 to bind to if it is not explicitly handled by any prior arm. The
	compiler sees this and says this is a valid match block because all cases
	have been handled.

	We have access to a different specifier that indicates to rust we will not
	be using the value; if we aren't going to use the value, then any value can
	match it because it doesn't matter. This is _. We can rewrite the previous
	code block as this, using _:*/

	let dice_roll = 9;
	match dice_roll {
		3 => add_fancy_hat(),
		7 => remove_fancy_hat(),
		_ => reroll(),
	}

	fn add_fancy_hat() {}
	fn remove_fancy_hat() {}
	fn reroll() {}

	/*

	In the above code, if dice_roll is not a 3 or a 7 (it isn't!) then it gets
	captured by _ and the program executes reroll. Note that unlike move_player
	from the previous example, reroll does not accept parameters, because _
	does not bind to a value and is thus not a meaningful parameter!

	We can also use _ in conjunction with () - the unit value. This indicates
	to the compiler that if we do not hit a match, we will not execute any more
	code. It might look something like this:*/

	let dice_roll = 9;
	match dice_roll {
		3 => add_fancy_hat(),
		7 => remove_fancy_hat(),
		_ => (),
	}

	fn add_fancy_hat() {}
	fn remove_fancy_hat() {}

	/*

	So we've got some powerful ways to set up match blocks; we're provided a
	lot of flexibility with this tool.

if let:

	Say we have an enum variant that may be one of many different things, and 
	we want to execute some code in the case of one specific variant, but not
	in the case of any others. Using match, it would look like this:*/

	let enum_variant = SomeEnum::Variant;
	match enum_variant {
		SomeEnum::Variant => println!("Do work with desired variant."),
		_ => (),
	}

	/*

	This is useful, but also clearly very annoying. So, we are given the if let
	syntax: Do something if the enum variant is exactly what we want, and skip
	it otherwise. The above code could look like this instead:*/

	let enum_variant = SomeEnum::Variant;
	if let SomeEnum::Variant = enum_variant {
		println!("Do work with desired variant.");
	}

	/*

	And that's it! It is exactly the same as writing a match arm except some 
	things have been moved around. We can also include an else block to handle
	the cases represented by _:*/

	let mut count = 0;
	if let Coin::Quarter(state) = coin {
		println!("State quarter from {:?}!", state);
	} else {
		count += 1;
	}

	/*

	And that's about it for enums. Being able to express a lot of different
	ideas in this flexible way, from the enums and variants themselves, to the
	implication of enum variants being complete types that may implement their
	own traits allowing for complicated behavior switching depending on type,
	to the very powerful match expressions provided us, make this seems like an
	integral part of programming in Rust. Certainly Option<T> and Result<T>
	show up everywhere, so I'm probably not too far off the mark.
