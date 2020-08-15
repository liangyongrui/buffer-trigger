use async_std::{
    sync::{Mutex, Receiver, RwLock, Sender},
    task,
};
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
    /// get and clear container
    get_and_clear_container: fn(&mut Option<P>) -> C,
}

/// General `BufferTrigger`
///
/// Set your own container to store in the current service
pub struct General<E, C, P>
where
    P: fmt::Debug + Sync + Send + 'static,
    E: fmt::Debug + Sync + Send + 'static,
    C: fmt::Debug + Sync + Send + 'static,
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
    P: fmt::Debug + Sync + Send,
    E: fmt::Debug + Sync + Send,
    C: fmt::Debug + Sync + Send,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name {}", self.name)
    }
}

impl<E, C, P> General<E, C, P>
where
    P: fmt::Debug + Sync + Send,
    E: fmt::Debug + Sync + Send,
    C: fmt::Debug + Sync + Send,
{
    pub async fn len(&self) -> usize {
        let c = self.locker.read().await;
        (c.get_len)(&c.payload)
    }
    pub async fn push(&self, value: E) {
        {
            let mut c = self.locker.write().await;
            (c.incr_len)(&mut c.payload);
            (c.accumulator)((c.get_container)(&mut c.payload), value);
            if let (false, Some(dur)) = (c.clock, self.interval) {
                c.clock = true;
                let sender = self.sender.lock().await.clone();
                let _ = task::spawn(async move {
                    task::sleep(dur).await;
                    sender.send(()).await
                });
            }
        }
        if self.len().await >= self.max_len {
            self.trigger().await
        }
    }

    pub async fn trigger(&self) {
        if !self.is_empty().await {
            let mut c = self.locker.write().await;
            c.clock = false;
            (c.clear_len)(&mut c.payload);
            (self.consumer)((c.get_and_clear_container)(&mut c.payload));
        }
    }

    pub async fn is_empty(&self) -> bool {
        self.len().await == 0
    }

    /// start clock trigger listener
    pub async fn listen_clock_trigger(&self) {
        log::info!("{:?} listen_clock_trigger", self);
        while self.receiver.lock().await.recv().await.is_ok() {
            let clock = self.locker.read().await.clock;
            if clock {
                self.trigger().await;
            }
        }
    }
}

impl<E, C, P> Drop for General<E, C, P>
where
    P: fmt::Debug + Sync + Send,
    E: fmt::Debug + Sync + Send,
    C: fmt::Debug + Sync + Send,
{
    fn drop(&mut self) {
        let _ = self.trigger();
    }
}
