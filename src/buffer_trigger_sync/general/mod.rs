use super::BufferTrigger;
use std::sync::{
    mpsc::{Receiver, Sender},
    Mutex, RwLock,
};
use std::thread;
use std::{fmt, time::Duration};

pub mod builder;
struct Locker<E, C, P>
where
    P: fmt::Debug,
    E: fmt::Debug,
    C: fmt::Debug,
{
    payload: Option<P>,
    /// Whether the timed task has been set
    clock: bool,
    /// Number of container elements
    get_len: fn(&Option<P>) -> usize,

    incr_len: fn(&mut Option<P>),

    clear_len: fn(&mut Option<P>),

    get_container: fn(&mut Option<P>) -> &mut C,
    /// accumulator function
    accumulator: fn(&mut C, E),
    get_and_clear_container: fn(&mut Option<P>) -> C,
}

/// General `BufferTrigger`
///
/// Set your own container to store in the current service
pub struct General<E, C, P>
where
    P: fmt::Debug + Send + 'static,
    E: fmt::Debug + Send + 'static,
    C: fmt::Debug + Send + 'static,
{
    name: String,
    locker: RwLock<Locker<E, C, P>>,
    /// The function executed after the trigger condition is met.
    consumer: fn(C),
    /// how many elements are exceeded
    max_len: usize,
    /// The maximum time to wait after an element is saved.
    interval: Option<Duration>,
    sender: Mutex<Sender<()>>,
    receiver: Mutex<Receiver<()>>,
}

impl<E, C, P> fmt::Debug for General<E, C, P>
where
    P: fmt::Debug + Send + 'static,
    E: fmt::Debug + Send + 'static,
    C: fmt::Debug + Send + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name {}", self.name)
    }
}

impl<E, C, P> super::BufferTrigger<E> for General<E, C, P>
where
    P: fmt::Debug + Send,
    E: fmt::Debug + Send,
    C: fmt::Debug + Send,
{
    fn len(&self) -> usize {
        if let Ok(c) = self.locker.read() {
            (c.get_len)(&c.payload)
        } else {
            0
        }
    }
    fn push(&self, value: E) {
        if let Ok(mut c) = self.locker.write() {
            (c.incr_len)(&mut c.payload);
            (c.accumulator)((c.get_container)(&mut c.payload), value);
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
            if let Ok(mut c) = self.locker.write() {
                c.clock = false;
                (c.clear_len)(&mut c.payload);
                (self.consumer)((c.get_and_clear_container)(&mut c.payload));
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<E, C, P> General<E, C, P>
where
    P: fmt::Debug + Send,
    E: fmt::Debug + Send,
    C: fmt::Debug + Send,
{
    /// start clock trigger listener
    fn listen_clock_trigger(&self) {
        log::info!("{:?} listen_clock_trigger", self);
        while let Ok(recevier) = self.receiver.lock() {
            if recevier.recv().is_ok() {
                let clock = if let Ok(c) = self.locker.read() {
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
impl<E, C, P> Drop for General<E, C, P>
where
    P: fmt::Debug + Send,
    E: fmt::Debug + Send,
    C: fmt::Debug + Send,
{
    fn drop(&mut self) {
        self.trigger();
    }
}
