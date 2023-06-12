/*

These are my notes on Chapter 1 of The Book.

We start by covering some of the tools available.

	Rust programs are indicated with the .rs extension. Rust naming convention
	assumes underscores for spaces in names.

	Rust programs run from their main() function, designated like so:*/

	fn main() {
		println!("Hello, world!");
	}

	/*

	Very simple. We compile this code with the rust compiler, terminal command
	rustc <filename>. This produces a binary that we can run. We can also do
	formatting/linting with the rustfmt command line tool. rustc is a good tool
	for simple (single file, few imports) projects; for larger projects, we need
	something more powerful.

	Rust has a build system and package manager, Cargo. It has functionality from
	to pip, or a per-project pacman, and will handle using the things it installs
	when you build your project.

	We can create a new project with*/

	$ cargo new hello_cargo

	/*

	This initializes a directory ./hello_cargo with a few things. It gives us a
	git repository with a file named Cargo.toml, a file named main.rs, a src
	directory, and a .gitignore populated with Cargo.toml. TOML stands for
	Tom's Obvious, Minimal Language, and is a config file format. It looks like:*/

	[package]
	name = "hello_cargo"
	version = "0.1.0"
	edition = "2021"

	[dependencies]

	/*

	Which I agree, is pretty obvious and minimal. Thanks, Tom. When we want to
	build a project, we can*/ 

	$ cargo build. 

	/*
	
	This creates an executable in the
	target subdirectory. It also creates a Cargo.lock file on successful build;
	this file records the highest version of each dependency used, so that if the
	program encounters errors when a package is updated, we know what version to
	roll back to. Other useful commands:*/ 

	$ cargo run //builds & executes; 
	$ cargo check /*runs the compiler without producing a binary to quickly allow
	you to error check your code. 

	That's it for basic setup stuff.