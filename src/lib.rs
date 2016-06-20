#![feature(plugin)]
#![plugin(peg_syntax_ext)]
extern crate cairo;
extern crate gtk;
extern crate gdk;
extern crate cgmath;

pub type Float = f64;
pub use ::std::f64::INFINITY;
pub use ::std::f64::NEG_INFINITY;

pub mod render;
pub mod types;
pub mod primitive;
pub mod xplicit_widget;
pub mod editor;
pub mod openscad;
