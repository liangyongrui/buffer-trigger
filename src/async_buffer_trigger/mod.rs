use async_std::{
    sync::{Mutex, Receiver, RwLock, Sender},
    task,
};
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
pub struct AsyncBufferTrigger<E, C>
where
    E: fmt::Debug + Sync + Send,
    C: fmt::Debug + Sync + Send,
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

impl<E, C> fmt::Debug for AsyncBufferTrigger<E, C>
where
    E: fmt::Debug + Sync + Send,
    C: fmt::Debug + Sync + Send,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name {}", self.name)
    }
}

impl<E, C> AsyncBufferTrigger<E, C>
where
    E: fmt::Debug + Sync + Send,
    C: fmt::Debug + Sync + Send,
{
    pub async fn len(&self) -> usize {
        self.container.read().await.len
    }

    pub async fn push(&self, value: E) {
        {
            let mut c = self.container.write().await;
            (c.accumulator)(&mut c.container, value);
            c.len += 1;
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
            let mut c = self.container.write().await;
            c.len = 0;
            let mut new_cahce = (self.defalut_container)();
            mem::swap(&mut new_cahce, &mut c.container);
            c.clock = false;
            (self.consumer)(new_cahce);
        }
    }

    pub async fn is_empty(&self) -> bool {
        self.len().await == 0
    }
}

impl<E, C> AsyncBufferTrigger<E, C>
where
    E: fmt::Debug + Sync + Send,
    C: fmt::Debug + Sync + Send,
{
    /// start clock trigger listener
    pub async fn listen_clock_trigger(&self) {
        log::info!("{:?} listen_clock_trigger", self);
        while self.receiver.lock().await.recv().await.is_ok() {
            if self.container.read().await.clock {
                self.trigger().await;
            }
        }
    }
}

impl<E, C> Drop for AsyncBufferTrigger<E, C>
where
    E: fmt::Debug + Sync + Send,
    C: fmt::Debug + Sync + Send,
{
    fn drop(&mut self) {
        let _ = self.trigger();
    }
}
