# learning-rust

Repository dedicated to learn Rust following [rust book](https://doc.rust-lang.org/stable/book/).

The main idea is not read the entire book in one go, a very detailed introduction to the basic concepts can be found until chapter 10.

## Cargo & VCS

Cargo is Rust’s build system and package manager, whenever we want to start a rust project we can run `cargo new <name>`. This will automatically generate a rust project boilerplate using git version control system (VCS), since all files will be stored in this repository we can skip vcs creation using `cargo new <name> --vcs none`. 

## Naming convention

Whenever the book presents a new topic we are going to name the project based on the html endpoint. For example, the hello world program using cargo is presented in the page [ch01-03-hello-cargo](https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html), then the project will be created with the command `cargo new ch01-03-hello-cargo --vcs none`.
