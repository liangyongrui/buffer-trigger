use super::AsyncBufferTrigger;
use super::Container;
use async_std::sync::{Mutex, RwLock};
use std::{fmt, time::Duration};
/// simple buffer trigger builer
pub struct Builder<E, C>
where
    E: fmt::Debug + Sync + Send,
{
    name: String,
    defalut_container: fn() -> C,
    accumulator: fn(&mut C, E),
    consumer: fn(C),
    max_len: usize,
    interval: Option<Duration>,
}

impl<E, C> fmt::Debug for Builder<E, C>
where
    E: fmt::Debug + Sync + Send,
    C: fmt::Debug + Sync + Send,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name {}", self.name)
    }
}

impl<E, C> Builder<E, C>
where
    E: fmt::Debug + Sync + Send,
    C: fmt::Debug + Sync + Send,
{
    /// init
    pub fn builder(defalut_container: fn() -> C) -> Self {
        Self {
            name: "anonymous".to_owned(),
            defalut_container,
            accumulator: |_, _| {},
            consumer: |_| {},
            max_len: std::usize::MAX,
            interval: None,
        }
    }

    /// set `name`
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    /// set `accumulator`
    pub fn accumulator(mut self, accumulator: fn(&mut C, E) -> ()) -> Self {
        self.accumulator = accumulator;
        self
    }

    /// set `consumer`
    pub fn consumer(mut self, consumer: fn(C)) -> Self {
        self.consumer = consumer;
        self
    }

    /// set `max_len`
    pub fn max_len(mut self, max_len: usize) -> Self {
        self.max_len = max_len;
        self
    }

    /// set `interval`
    pub fn interval(mut self, interval: Duration) -> Self {
        self.interval = Some(interval);
        self
    }

    /// `build`
    pub fn build(self) -> AsyncBufferTrigger<E, C> {
        let (sender, receiver) = async_std::sync::channel(10);
        AsyncBufferTrigger {
            name: self.name,
            defalut_container: self.defalut_container,
            container: RwLock::new(Container {
                len: 0,
                accumulator: self.accumulator,
                container: (self.defalut_container)(),
                clock: false,
            }),
            consumer: self.consumer,
            max_len: self.max_len,
            interval: self.interval,
            sender: Mutex::new(sender),
            receiver: Mutex::new(receiver),
        }
    }
}
