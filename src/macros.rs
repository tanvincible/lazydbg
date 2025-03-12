//! Macro definitions for `lazydbg`.

/// A debug macro that prints only in debug mode, removed in release mode.
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! lazydbg {
    ($($arg:tt)*) => {
        dbg!($($arg)*)
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! lazydbg {
    ($($arg:tt)*) => {
        // Does nothing in release mode
    };
}
