//Until it's clear what the unstable things are replaced by
#![allow(unstable)]
#![allow(non_snake_case)]

#[macro_use] extern crate log;
extern crate "rustc-serialize" as rustc_serialize;
extern crate core;
extern crate hyper;
extern crate regex;


#[macro_use] pub mod macros;
mod messagebuilder;
pub mod command;
pub mod common;
pub mod error;
pub mod httpserver;
pub mod response;


#[test]
fn it_works() {
}
