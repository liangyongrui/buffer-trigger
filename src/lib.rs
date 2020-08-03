//! buffer trigger

#![deny(
    clippy::all,
    clippy::correctness,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(clippy::must_use_candidate)]

/// common trait
pub trait BufferTrigger<T> {
    /// is empty
    fn is_empty(&self) -> bool;

    /// The number of elements in  `BufferTrigger`
    fn len(&self) -> usize;

    /// add elements
    fn push(&self, value: T);

    /// Manual trigger
    fn trigger(&self);
}

mod async_buffer_trigger;
pub use async_buffer_trigger::builder::Builder as AsyncBufferTriggerBuilder;
pub use async_buffer_trigger::AsyncBufferTrigger;

mod simple_buffer_trigger;
pub use simple_buffer_trigger::builder::Builder as SimpleBufferTriggerBuilder;
pub use simple_buffer_trigger::SimpleBufferTrigger;
