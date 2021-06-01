//! # LibNU
//! LibNU is the Novusk User library, it has all the crates, functions, structs, types, etc used for
//! developing an operating system or an app for a Novusk based OS.
//!
//! ### Re-export usages
//!
//! uefi_graphics (https://github.com/NathanMcMillan54/uefi_graphics/) should be used but isn't
//! because crates.io doesn't allow git dependencies. It's recommended that is is imported as
//! "graphics" in your OS.
//!
//! uefi has some structs an enum types re-exported for file I/O in io/io.fs
//!
//! ctypes is re-exported from Novusk for make C-equivalent variables and values. It is found it
//! ktypes which is used as "types".

#![no_std]

pub mod desktop;

#[macro_use]
pub mod io;

pub mod ktypes;
pub use ktypes as types;
