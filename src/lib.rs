//! Core abstractions of the Real Time For the Masses (RTFM) framework
//!
//! You can write generic *libraries* by using the `Resource` trait in this
//! crate. If you want to write application code then you'll need an
//! implementation of the RTFM framework for a particular architecture.
//! Currently there are implementations for these two architectures:
//!
//! - [ARM Cortex-M](https://crates.io/crates/cortex-m-rtfm)
//! - [MSP430](https://crates.io/crates/msp430-rtfm)
#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

use core::u8;
use core::marker::PhantomData;

/// A resource, a mechanism to share data between tasks
pub unsafe trait Resource {
    /// The data protected by the resource
    type Data: Send;

    /// Borrows the resource data for the duration of a critical section
    ///
    /// # Panics
    ///
    /// This will `panic!` if the threshold is not high enough to protect the
    /// resource data from data races
    fn borrow<'cs>(&'cs self, t: &'cs Threshold) -> &'cs Self::Data;

    /// Mutable variant of `borrow`
    fn borrow_mut<'cs>(&'cs mut self, t: &'cs Threshold) -> &'cs mut Self::Data;

    /// Claims the resource data for the span of the closure `f`. For the
    /// duration of the closure other tasks that may access the resource data
    /// are prevented from preempting the current task.
    fn claim<R, F>(&self, t: &mut Threshold, f: F) -> R
    where
        F: FnOnce(&Self::Data, &mut Threshold) -> R;

    /// Mutable variant of `claim`
    fn claim_mut<R, F>(&mut self, t: &mut Threshold, f: F) -> R
    where
        F: FnOnce(&mut Self::Data, &mut Threshold) -> R;
}

/// Preemption threshold token
///
/// The preemption threshold indicates the priority a task must have to preempt
/// the current context. For example a threshold of 2 indicates that only
/// interrupts / exceptions with a priority of 3 or greater can preempt the
/// current context
pub struct Threshold {
    value: u8,
    _not_send: PhantomData<*const ()>,
}

impl Threshold {
    /// Creates a new `Threshold` token
    ///
    /// This API is meant to be used to create abstractions and not to be
    /// directly used by applications.
    pub unsafe fn new(value: u8) -> Self {
        Threshold {
            value,
            _not_send: PhantomData,
        }
    }

    /// Creates a `Threshold` token with maximum value
    ///
    /// This API is meant to be used to create abstractions and not to be
    /// directly used by applications.
    pub unsafe fn max() -> Self {
        Self::new(u8::MAX)
    }

    /// Returns the value of this `Threshold` token
    pub fn value(&self) -> u8 {
        self.value
    }
}
