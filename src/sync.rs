//! Taking a lock without letting one panic end the process's usefulness.

use std::sync::{Mutex, MutexGuard};

/// Lock `mutex`, recovering if a previous holder panicked.
///
/// A poisoned mutex means some thread panicked while holding it. Rust's
/// default — every later `lock()` returns an error, and `expect` turns that
/// into another panic — is the right default for data whose invariants a torn
/// write would break. It is the wrong one here.
///
/// The workbench is thread-per-connection over shared state. Everything behind
/// these mutexes is either a timestamp or a set of in-flight run identifiers;
/// nothing has an invariant that spans two writes. Refusing to look at them
/// after one handler panics turns a single failed request into a service that
/// answers nothing for the rest of its life, with a browser tab that hangs and
/// no error anywhere the learner can see. Recovering is strictly better.
///
/// The pattern already existed in `tests/browser_journey.rs`; it was never
/// applied to the service it tests.
pub fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}
