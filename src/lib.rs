#![feature(exact_size_is_empty)]
#![feature(plugin)]
#![plugin(bifrost_plugins)]

#[macro_use]
pub mod types;
pub mod expr;
pub mod lexer;

extern crate bincode;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bifrost;
extern crate bifrost_hasher;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;