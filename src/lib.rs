#![license = "MIT"]
#![deny(missing_docs)]
#![deny(warnings)]

//! Intertwine an Iterator of Iterators.

use std::comm::Receiver;

/// An Iterator which iterates over the values from many Iterators.
pub struct Intertwined<T: Send> {
    receiver: Receiver<T>
}

/// A mixin trait for the `intertwine` method, which constructs Intertwined values.
pub trait Intertwine<T> {
    /// Intertwine an Iterator of Iterators
    fn intertwine(self) -> Intertwined<T>;
}

impl<T: Send, I: Iterator<T> + Send, II: Iterator<I>> Intertwine<T> for II {
    fn intertwine(mut self) -> Intertwined<T> {
        let (tx, rx) = channel();

        for mut iterator in self {
            let tx = tx.clone();
            spawn(proc() {
                for x in iterator {
                    if tx.send_opt(x).is_err() { break }
                }
            });
        }

        Intertwined {
            receiver: rx
        }
    }
}

impl<T: Send> Iterator<T> for Intertwined<T> {
    fn next(&mut self) -> Option<T> { self.receiver.recv_opt().ok() }
}

