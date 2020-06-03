//! Core abstractions of the Real Time For the Masses (RTFM) framework
//!
//! You can write generic *libraries* using the `Mutex` trait in this crate. If you want to write
//! application code then you'll need an *implementation* of the RTFM framework for a particular
//! architecture. Currently, there are implementations for these architectures and OSes:
//!
//! - [ARM Cortex-M](https://crates.io/crates/cortex-m-rtfm)
// - [Linux]
// - [MSP430]
// - [RISC-V]

#![deny(missing_docs)]
#![deny(rust_2018_compatibility)]
#![deny(rust_2018_idioms)]
#![deny(warnings)]
#![no_std]

pub use mutex_trait::{prelude::*, Exclusive, Mutex};
pub use embedded_time as time;

/// A monotonic clock / counter
pub trait Monotonic: time::Clock {
    /// Resets the counter to *zero*
    ///
    /// # Safety
    ///
    /// This function will be called *exactly once* by the RTFM runtime after `#[init]` returns and
    /// before tasks can start; this is also the case in multi-core applications. User code must
    /// *never* call this function.
    unsafe fn reset();
}

/// A marker trait that indicates that it is correct to use this type in multi-core context
pub trait MultiCore {}
