use crate::BufferTrigger;
use std::sync::{
    mpsc::{Receiver, Sender},
    Mutex, RwLock,
};
use std::thread;
use std::{fmt, mem, time::Duration};

pub mod builder;
struct Container<E, C> {
    /// Whether the timed task has been set
    clock: bool,
    /// Number of container elements
    len: usize,
    container: C,
    /// accumulator function
    accumulator: fn(&mut C, E) -> (),
}

/// Simple `BufferTrigger`
///
/// Set your own container to store in the current service
pub struct SimpleBufferTrigger<E, C>
where
    E: fmt::Debug,
    C: fmt::Debug,
{
    name: String,
    /// The default generation function of the container
    defalut_container: fn() -> C,
    container: RwLock<Container<E, C>>,
    /// The function executed after the trigger condition is met.
    consumer: fn(C) -> (),
    /// how many elements are exceeded
    max_len: usize,
    /// The maximum time to wait after an element is saved.
    interval: Option<Duration>,

    sender: Mutex<Sender<()>>,
    receiver: Mutex<Receiver<()>>,
}

impl<E, C> fmt::Debug for SimpleBufferTrigger<E, C>
where
    E: fmt::Debug,
    C: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name {}", self.name)
    }
}

impl<E, C> BufferTrigger<E> for SimpleBufferTrigger<E, C>
where
    E: fmt::Debug,
    C: fmt::Debug,
{
    fn len(&self) -> usize {
        if let Ok(c) = self.container.read() {
            c.len
        } else {
            0
        }
    }
    fn push(&self, value: E) {
        if let Ok(mut c) = self.container.write() {
            (c.accumulator)(&mut c.container, value);
            c.len += 1;
            if let (false, Some(dur)) = (c.clock, self.interval) {
                c.clock = true;
                match self.sender.lock() {
                    Ok(sender) => {
                        let sender = sender.clone();
                        let _ = thread::spawn(move || {
                            thread::sleep(dur);
                            if let Err(e) = sender.send(()) {
                                log::error!("auto clock trigger error {}", e);
                            };
                        });
                    }
                    Err(e) => {
                        log::error!("{}", e);
                    }
                }
            }
        }
        if self.len() >= self.max_len {
            self.trigger()
        }
    }

    fn trigger(&self) {
        if !self.is_empty() {
            if let Ok(mut c) = self.container.write() {
                c.len = 0;
                let mut new_cahce = (self.defalut_container)();
                mem::swap(&mut new_cahce, &mut c.container);
                c.clock = false;
                (self.consumer)(new_cahce);
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<E, C> SimpleBufferTrigger<E, C>
where
    E: fmt::Debug,
    C: fmt::Debug,
{
    /// start clock trigger listener
    pub fn listen_clock_trigger(&self) {
        log::info!("{:?} listen_clock_trigger", self);
        while let Ok(recevier) = self.receiver.lock() {
            if recevier.recv().is_ok() {
                let clock = if let Ok(c) = self.container.read() {
                    c.clock
                } else {
                    false
                };
                if clock {
                    self.trigger();
                }
            }
        }
    }
}

impl<E, C> Drop for SimpleBufferTrigger<E, C>
where
    E: fmt::Debug,
    C: fmt::Debug,
{
    fn drop(&mut self) {
        self.trigger();
    }
}
