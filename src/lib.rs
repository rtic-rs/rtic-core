//! Core abstractions of the Real-Time Interrupt-driven Concurrency (RTIC) framework
//!
//! You can write generic *libraries* using the `Mutex` trait in this crate. If you want to write
//! application code then you'll need an *implementation* of the RTIC framework for a particular
//! architecture. Currently, there are implementations for these architectures and OSes:
//!
//! - [ARM Cortex-M](https://crates.io/crates/cortex-m-rtic)
// - [Linux]
// - [MSP430]
// - [RISC-V]

#![deny(missing_docs)]
#![deny(rust_2018_compatibility)]
#![deny(rust_2018_idioms)]
#![deny(warnings)]
#![no_std]

use core::ops;

/// Memory safe access to shared resources
///
/// In RTIC, locks are implemented as critical sections that prevent other tasks from *starting*.
/// These critical sections are implemented by temporarily increasing the dynamic priority of the
/// current context. Entering and leaving these critical sections is always done in bounded constant
/// time (a few instructions in bare metal contexts).
pub trait Mutex {
    /// Data protected by the mutex
    type T;

    /// Creates a critical section and grants temporary access to the protected data
    fn lock<R>(&mut self, f: impl FnOnce(&mut Self::T) -> R) -> R;
}

impl<'a, M> Mutex for &'a mut M
where
    M: Mutex,
{
    type T = M::T;

    fn lock<R>(&mut self, f: impl FnOnce(&mut M::T) -> R) -> R {
        M::lock(self, f)
    }
}

/// Newtype over `&'a mut T` that implements the `Mutex` trait
///
/// The `Mutex` implementation for this type is a no-op: no critical section is created
pub struct Exclusive<'a, T>(pub &'a mut T);

impl<'a, T> Mutex for Exclusive<'a, T> {
    type T = T;

    fn lock<R>(&mut self, f: impl FnOnce(&mut T) -> R) -> R {
        f(self.0)
    }
}

impl<'a, T> ops::Deref for Exclusive<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.0
    }
}

impl<'a, T> ops::DerefMut for Exclusive<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0
    }
}

/// Makes locks work on N-tuples, locks the mutexes from left-to-right in the tuple. These are
/// used to reduce rightward drift in code and to help make intentions clearer.
///
/// # Example
///
/// ```
/// use mutex_trait::prelude::*;
///
/// fn normal_lock(
///     a: &mut impl Mutex<T = i32>,
///     b: &mut impl Mutex<T = i32>,
///     c: &mut impl Mutex<T = i32>
/// ) {
///     // A lot of rightward drift...
///     a.lock(|a| {
///         b.lock(|b| {
///             c.lock(|c| {
///                 *a += 1;
///                 *b += 1;
///                 *c += 1;
///             });
///         });
///     });
/// }
/// ```
///
/// Has a shorthand as:
///
/// ```
/// use mutex_trait::prelude::*;
///
/// fn tuple_lock(
///     a: &mut impl Mutex<T = i32>,
///     b: &mut impl Mutex<T = i32>,
///     c: &mut impl Mutex<T = i32>
/// ) {
///     // Look! Single indent and less to write
///     (a, b, c).lock(|a, b, c| {
///         *a += 1;
///         *b += 1;
///         *c += 1;
///     });
/// }
/// ```
pub mod prelude {
    pub use crate::Mutex;

    macro_rules! lock {
        ($e:ident, $fun:block) => {
            $e.lock(|$e| $fun )
        };
        ($e:ident, $($es:ident),+, $fun:block) => {
            $e.lock(|$e| lock!($($es),*, $fun))
        };
    }

    macro_rules! make_tuple_impl {
        ($name:ident, $($es:ident),+) => {
            /// Auto-generated tuple implementation, see [`Mutex`](../trait.Mutex.html) for details.
            pub trait $name {
                $(
                    /// Data protected by the mutex.
                    type $es;
                )*

                /// Creates a critical section and grants temporary access to the protected data.
                fn lock<R>(&mut self, f: impl FnOnce($(&mut Self::$es),*) -> R) -> R;
            }

            impl<$($es),+> $name for ($($es,)+)
            where
                $($es: crate::Mutex),*
            {
                $(
                    type $es = $es::T;
                )*

                #[allow(non_snake_case)]
                fn lock<R>(&mut self, f: impl FnOnce($(&mut Self::$es),*) -> R) -> R {
                    let ($(
                        $es,
                    )*) = self;

                    lock!($($es),*, { f($($es),*) })
                }
            }
        };
    }

    // Generate tuple lock impls
    make_tuple_impl!(TupleExt01, T1);
    make_tuple_impl!(TupleExt02, T1, T2);
    make_tuple_impl!(TupleExt03, T1, T2, T3);
    make_tuple_impl!(TupleExt04, T1, T2, T3, T4);
    make_tuple_impl!(TupleExt05, T1, T2, T3, T4, T5);
    make_tuple_impl!(TupleExt06, T1, T2, T3, T4, T5, T6);
    make_tuple_impl!(TupleExt07, T1, T2, T3, T4, T5, T6, T7);
    make_tuple_impl!(TupleExt08, T1, T2, T3, T4, T5, T6, T7, T8);
    make_tuple_impl!(TupleExt09, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    make_tuple_impl!(TupleExt10, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    make_tuple_impl!(TupleExt11, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    make_tuple_impl!(TupleExt12, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
}

