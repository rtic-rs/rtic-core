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

use core::ops::Sub;
pub use mutex_trait::{prelude::*, Exclusive, Mutex};

/// A fraction
pub struct Fraction {
    /// The numerator
    pub numerator: u32,

    /// The denominator
    pub denominator: u32,
}

/// A monotonic clock / counter
pub trait Monotonic {
    /// A measurement of this clock, use `CYCCNT` as a reference implementation for `Instant`.
    /// Note that the Instant must be a signed value such as `i32`.
    type Instant: Copy + Ord + Sub;

    /// The ratio between the system timer (SysTick) frequency and this clock frequency, i.e.
    /// `Monotonic clock * Fraction = System clock`
    ///
    /// The ratio must be expressed in *reduced* `Fraction` form to prevent overflows. That is
    /// `2 / 3` instead of `4 / 6`
    fn ratio() -> Fraction;

    /// Returns the current time
    ///
    /// # Correctness
    ///
    /// This function is *allowed* to return nonsensical values if called before `reset` is invoked
    /// by the runtime. Therefore application authors should *not* call this function during the
    /// `#[init]` phase.
    fn now() -> Self::Instant;

    /// Resets the counter to *zero*
    ///
    /// # Safety
    ///
    /// This function will be called *exactly once* by the RTFM runtime after `#[init]` returns and
    /// before tasks can start; this is also the case in multi-core applications. User code must
    /// *never* call this function.
    unsafe fn reset();

    /// A `Self::Instant` that represents a count of *zero*
    fn zero() -> Self::Instant;
}

/// A marker trait that indicates that it is correct to use this type in multi-core context
pub trait MultiCore {}
