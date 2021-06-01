//! # LibNU
//! LibNU is the Novusk User library, it has all the crates, functions, structs, types, etc used for
//! developing an operating system or an app for a Novusk based OS.
//!
//! ### Re-export usages
//!
//! uefi_graphics is re-exported as "graphics" for making user interfaces and displaying
//! images/shapes.
//!
//! uefi has some structs an enum types re-exported for file I/O in io/fs
//!
//! ctypes is re-exported from Novusk for make C-equivalent variables and values. It is found it
//! ktypes which is used as "types".

#![no_std]

pub extern crate desktop;

#[macro_use]
pub extern crate io;

extern crate ktypes;
pub use ktypes as types;

extern crate uefi_graphics;
pub use uefi_graphics as graphics;
