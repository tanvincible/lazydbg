//! `lazydbg` - A lightweight debugging macro that is removed in release mode.
//!
//! ## Usage
//! ```rust
//! use lazydbg::lazydbg;
//!
//! fn main() {
//!     let x = 42;
//!     lazydbg!(x); // Prints in debug mode, removed in release mode
//! }
//! ```

mod macros;
