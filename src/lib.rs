//! # LibNU
//! LibNU is the Novusk User library, it has all the crates, functions, structs, types, etc used for
//! developing an operating system or an app for a Novusk based OS.
//!
//! ### Re-export usages
//!
//! uefi_graphics (https://github.com/NathanMcMillan54/uefi_graphics/) should be used but isn't
//! because crates.io doesn't allow git dependencies.
//!
//! uefi has some structs an enum types re-exported for file I/O in io/io.fs
//!
//! ctypes (https://github.com/new-kernel/novusk, include/ctypes/) should be used but isn't
//! because crates.io doesn't allow git dependencies. It's recommended that you use it for
//! C-equivalent code.

#![no_std]

pub mod desktop;
pub mod graphics;

#[macro_use]
pub mod io;

pub mod ktypes;
pub use ktypes as types;
