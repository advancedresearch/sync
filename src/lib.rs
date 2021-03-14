#![deny(missing_docs)]

//! # Sync
//! A research project about synchronizability and cosynchronizability
//!
//! ### Motivation
//!
//! Equivalence classes in Outside theories of
//! [Avatar Extensions](https://advancedresearch.github.io/avatar-extensions/summary.html)
//! imposes weaker conditions than normalizability,
//! called *synchronizability* and *cosynchronizability*
//! (see [paper](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/synchronizability-and-cosynchronizability.pdf)).
//!
//! This research project explores these properties using a common abstract in Rust.
//!
//! ### Introduction to Synchronizability and Cosynchronizability
//!
//! In mathematics of sets, if `a = b` then we usually don't care about *how* they are equal.
//!
//! In [Homotopy Type Theory](https://homotopytypetheory.org/),
//! objects can be equal in the sense that there exists a continuous map
//! from one object to another. These maps can have continuous maps
//! between maps and so on. This is very useful for theorem proving about topological spaces. Using this intuition, two objects `a` and `b`
//! are equal if there exists a continuous map between them,
//! but this does not imply higher order maps.
//!
//! For example, on a torus, it is possible to create two loops which
//! can not be continuously transformed into each other.
//!
//! In higher dimensional mathematics, specially in the theory of
//! [Avatar Extensions](https://advancedresearch.github.io/avatar-extensions/summary.html),
//! mathematical objects might not have a unique identity,
//! which is a bit like transforming objects using continuous maps.
//! However, instead of building this structure directly using maps,
//! it is beneficial to take a step back and think of a weaker kind of transformation.
//!
//! When two objects are synchronized, it means that they both exist in a state
//! where they can be compared. With other words, the "time" which these
//! object exist is the same. So, when one object is brought to the "time"
//! of another object, they can be put in a relationship to each other,
//! while in different times this relationship has no meaning.
//!
//! Synchronizability and Cosynchronizability are two properties that makes
//! it easy to compare objects of some kind by bringing them to the same "time"
//! in a systematic way.
//!
//! - Synchronizability: An item that can be brought to the time of others
//! - Cosynchronizability: An item which time can bring other items to itself
//!
//! This way of thinking is slightly more flexible and perhaps more applicable
//! than homotopy maps, which might help build intuition about hihgher dimensional mathematics.

/// Implemented by objects that might be synchronized.
pub trait Synchronize: Sized + Eq {
    /// The type of "time".
    ///
    /// This is some information that tells how to synchronize objects
    /// so they can be checked for equivalence.
    type Time;

    /// Gets the time of some object.
    fn time(&self) -> Self::Time;

    /// Synchronizes the object with some time, if possible.
    fn synchronize(&self, time: &Self::Time) -> Option<Self>;

    /// Returns `true` if the object can be synchronized with some time.
    fn can_synchronize(&self, time: &Self::Time) -> bool {
        self.synchronize(time).is_some()
    }

    /// Returns `Some(true)` if the two objects are equivalent.
    /// Returns `Some(false)` if the two objects are inequivalent.
    /// Returns `None` if the two objects can not be synchronized.
    fn equiv(&self, b: &Self) -> Option<bool> {
        if let Some(b2) = b.synchronize(&self.time()) {
            Some(self == &b2)
        } else if let Some(a2) = self.synchronize(&b.time()) {
            Some(&a2 == b)
        } else {
            None
        }
    }
}

/// Implemented by efficient synchronizable or cosynchronizable equivalence classes.
///
/// The properties synchronizable or cosynchronizable means that
/// the equivalence class can be checked in `O(N)` when time is `O(1)`.
///
/// A `core` is chosen such that every item can easily relate to it.
/// When all items are equivalent to the `core`,
/// they are said to be "core-equivalent".
/// Core-equivalence implies equivalence under equivalence relations.
pub trait CoreEquiv: Iterator + Sized
    where Self::Item: Synchronize
{
    /// Returns the `core` of the equivalence class.
    fn core(&self) -> Self::Item;

    /// Returns `true` if the equivalence class is cosynchronizable.
    ///
    /// This means that any item can be synchronized to the time of the `core`.
    fn cosynchronizable() -> bool;

    /// Returns `true` if the equivalence class is synchronizable.
    ///
    /// This means that the `core` can be synchronized to the time
    /// of any other item.
    fn synchronizable() -> bool;

    /// Returns `true` if `a` is a member of the equivalence class.
    fn member(&self, a: &Self::Item) -> Option<bool> {self.core().equiv(a)}

    /// Returns `true` if all items of the iterator are equivalent.
    /// Returns `false` otherwise.
    fn check(mut self) -> bool {
        let core = self.core();
        if Self::cosynchronizable() {
            let core_time = core.time();
            self.all(|a| {
                if let Some(a2) = a.synchronize(&core_time) {
                    core == a2
                } else {
                    false
                }
            })
        } else if Self::synchronizable() {
            self.all(|a| {
                if let Some(core2) = core.synchronize(&a.time()) {
                    core2 == a
                } else {
                    false
                }
            })
        } else {
            false
        }
    }
}
