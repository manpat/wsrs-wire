#![feature(const_fn)]
#![feature(link_args)]
#![feature(box_syntax)]
#![feature(ord_max_min)]
#![feature(ascii_ctype)]
#![feature(inclusive_range_syntax)]

#[macro_use]
extern crate common;
extern crate libc;

extern crate png;

mod resources;

#[macro_use]
mod ems;
mod input;
mod context;
mod rendering;
mod connection;

mod ui;

mod player;
mod level;

use context::*;

pub mod res {
	pub use resources::*;
}

fn main() {
	println!("Is Hosted:      {}", cfg!(hosted));
	println!("Public address: {}", env!("PUBLIC_ADDRESS"));

	ems::start(Box::into_raw(box MainContext::new()));
}

