#![feature(plugin)]
#![plugin(peg_syntax_ext)]
extern crate cairo;
extern crate gtk;
extern crate gdk;
extern crate cgmath;

pub type Float = f64;
pub const INFINITY: Float = 1e10;
pub const NEG_INFINITY: Float = -1e10;

pub mod render;
pub mod types;
pub mod primitive;
pub mod xplicit_widget;
pub mod editor;
pub mod openscad;
pub mod menu;
pub mod mesh_view;
pub mod tessellation;
