use crate::BufferTrigger;
use std::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex, RwLock,
};
use std::thread;
use std::{fmt, mem, time::Duration};

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
            let clock = c.clock;
            if !clock {
                if let Some(dur) = self.interval {
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

/// simple buffer trigger builer
pub struct Builder<E, C>
where
    E: fmt::Debug,
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
    E: fmt::Debug,
    C: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name {}", self.name)
    }
}

impl<E, C> Builder<E, C>
where
    E: fmt::Debug,
    C: fmt::Debug,
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
    pub fn build(self) -> SimpleBufferTrigger<E, C> {
        let (sender, receiver) = mpsc::channel();
        SimpleBufferTrigger {
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
