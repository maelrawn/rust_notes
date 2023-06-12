/*

These are my notes on Chapter 2 of The Book.

This chapter appears to cover the basics of Rust syntax and program structure.
To do so, we'll write a program that takes user input to make a guessing game.
Because we are using Cargo to make a new project, this file will mirror the
comments in the actual main.rs file in a related directory.

The game:

	First, we run */

$ cargo new guessing_game

	/*

	to create a new directory with Cargo.toml, src/, and src/main.rs.

	Here is our initial code: */

	use std::io; //import standard io library into our code

	fn main() { //entry point into every Rust program

	    println!("Guess the number!"); //Macro that prints to screen

	    println!("Please input your guess."); //Same macro

	    let mut guess = String::new();	//Here, we create a String variable to
										//store user input using let. All Rust
										//variables are const by default, so we
										//also declare this string mut: mutable,
										//or non-const.

										//Also note the :: syntax indicates the
										//new function is implemented on a type.

	    io::stdin()				//Gets an instance of std::io::Stdin;
	        .read_line(&mut guess)
	       						//calls read_line on that instance and stores
	        					//the data in the provided string. Note we
	        					//pass a mutable reference here; if we didn't
								//pass by reference, the string would be
        						//consumed by the function and our program
        						//would fail since we use guess again later.
        						//The reference is declared mut because
        						//references are also immutable by default,
								//just like the variables they reference.
	        .expect("Failed to read line");
								//expect gives us a Result type. Result is an
								//enum with states Ok and Err. The Ok variant
								//indicates a successful operation and holds
								//the relevant data. The Err variant means
								//the operation failed, and Err contains info
								//as to why. If Err state is encountered, the
								//program will crash and print the message
								//passed to expect(). 

	    println!("You guessed: {guess}"); //This is just a python fstring.
	}

	/*

	So we've got the basic i/o part of our program down. We can't really call
	this a game yet, though; it doesn't have a goal or victory state. We need to
	generate a random number for that, and to do that we need to import a crate.

Importing Crates:

	Rust doesn't ship with a random number generator by default, but one exists
	within the Cargo ecosystem. It's called rand. We'll import it by editing our
	Cargo.toml dependencies with this line: */

	[dependencies]
	rand = "0.8.5"

	/*

	We've specified to Cargo that we want the crate rand with version 
	0.8.5 <= x < 0.9.0. This happens because versions are specified with Semantic
	Versioning in Cargo.toml (read more at https://semver.org). Any versions in
	this range are expected to have the same API, and should all be compatible
	with one another. Version 0.9.0 or higher do not provide this guarantee, so
	Cargo won't grab a version that high.

	Cargo gets these dependencies from Crates.io, which is a lot like arch linux'
	aur. Because of Cargo.lock, Cargo won't update dependencies without being
	explicitly told to do so. We can initiate that with */

	$ cargo update

	/*

	and Cargo will update to the highest version of each package that still fits
	in the range specified by your semver.

	Now that all the Cargo stuff is over, let's use that package. Digging through
	crates.io a little bit, we can find that its documentation lives at
	https://docs.rs/rand/0.8.5/rand/. This isn't wildly important to us right now,
	as The Book will walk us through using it for our intended purpose, but it is
	always useful to know where documentation lives.

	So, here's the second version of our code: */

use std::io;
use rand::Rng; 	//We add a new import for the trait we need

fn main() {
    println!("Guess the number!");

    //Here, thread_rng() is a function belonging to the rand crate, so we must
    //namespace it with ::. It gives us a random number generator to use. Then, 
    //gen_range() outputs a random number from the specified range. Ranges in 
    //rust are indicated with syntax x..y. They represent [x,y) unless 
    //written x..=y, which means [x,y].
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}.");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}

	/* 

	This kind of begs the question: How could we possibly know how to call some
	of this stuff? The answer is Cargo. Cargo will actually build and open doc
	pages in your browser for all specified dependencies in a given project via
	cargo doc --open.

Making comparisons and winning the game:

	We can't win the game if we can't compare the guess to the secret, so let's
	work on that. Let's add code for comparisons: */

use std::io;
use std::cmp::Ordering; //Here, we import the trait that allows comparisons
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}.");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    //Here we convert our string to an integer of type u32. Note that they use
    //the same name; the old value is 'shadowed' with this new one.
    let guess: u32 = guess.trim() 	//Strip whitespace/newline from string
    					.parse()	//Read number from string bytes
    					.expect("Please type a number!"); //Handle nonnumerics

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {	//Note the reference so as not to
    									//consume our secret. Now, let's look
    									//at what match actually does. We first
    									//call guess.cmp(&secret), which gives
    									//us an Ordering. Say the guess is 50
    									//and the secret is 38; we'll end up
    									//with an Ordering::Greater. Now, for
    									//each 'arm' of our match block, we
    									//check to see if the thing we have is
    									//exactly one of the arm patterns. We
    									//first check if Ordering::Greater is
    									//the same as Ordering::Less. It isn't,
    									//so we go to the next arm. We see that
    									//Ordering::Greater==Ordering::Greater,
    									//so we execute that code. We don't
    									//look at Ordering::Equal because match
    									//stops after the first success.
    	Ordering::Less => println!("Too small!"),
    	Ordering::Greater => println!("Too large!"),
    	Ordering::Equal => println!("You win!"),
    }
}

	/*

	Ok, we have comparisons. But it would kind of defeat the purpose to let the
	player know if they were over or under if they couldn't try again, right?
	So we can throw all the guessing code into a loop:*/

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}.");

    loop { //This is like writing while(1){...} in cpp
    	
	    println!("Please input your guess.");

	    let mut guess = String::new();

	    io::stdin()
	        .read_line(&mut guess)
	        .expect("Failed to read line");

	    let guess: u32 = guess.trim() 
	    					.parse()	
	    					.expect("Please type a number!");

	    println!("You guessed: {guess}");

	    match guess.cmp(&secret_number) {	
	    	Ordering::Less => println!("Too small!"),
	    	Ordering::Greater => println!("Too large!"),
	    	Ordering::Equal => {
	    		println!("You win!");
	    		break; 	//Because we basically wrote while(1) earlier, we need
	    				//a way for the program to exit. The break statement
	    				//does that, as we would expect from other languages.
	    	}
	    }
	}
}

	/*

	Our game is now functional! We can guess and guess until we get it or get
	bored. But what if the user accidentally inputs a nonnumeric character? The
	program would crash with message "Please type a number!" and the user would
	never emotionally recover from having lost their guessing game. So, we can
	add some error handling code:

Handling Invalid Input, or How To Use Result, For Real:

	Because Result is an enum, we can do some more refined error handling based
	on its state in places where we could call expect(). Check this out:*/

	// Not copying the whole program again

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() { //Execute match on the Result
        Ok(num) => num,		//If Result is in Ok state, return the value held
        					//by the Ok wrapper. Note 'num' could be any name;
        					//it is not actually specifying a type but that the
        					//Ok state is holding something.

        Err(_) => continue, //If Result is in error state, return to loop start
        					//so the user can be prompted for another guess.
        					//The _ parameter here denotes a catchall, so any
        					//error produced by parse() would fit into this
        					//category. If we wanted to catch specific errors
        					//and handle them differently, we could make more
        					//arms with more specialized parameters in our
        					//match block.
    };

    /*

    And that's it! We've successfully taken input, handled the possible error
    state, processed our data, and added a control structure that makes sense
    for our game. The full, final code is located at guessing_game/src/main.rs.

    Personally, I feel like the most impactful sections here are the use of
    match, and the use of Result. Match is clearly analogous to switch() in C,
    but feels a little more powerful because of traits. And then the Result
    type, through this example, stops being cumbersome and becomes something
    clearly extremely useful. It's like try, catch, finally in java but
    packaged in a way that makes it convenient to use. It feels pretty good.