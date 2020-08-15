pub(crate) mod general;
pub(crate) mod simple;

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

pub use general::builder::Builder as GeneralBuilder;
pub use general::General;

pub use simple::Builder as SimpleBuilder;
pub use simple::Simple;
