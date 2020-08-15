use super::{General, Locker};
use async_std::sync::{self, Mutex, RwLock};
use lifetime_thread::Outer;
use std::{fmt, time::Duration};
/// general buffer trigger builer
pub struct Builder<E, C, P>
where
    P: fmt::Debug,
    E: fmt::Debug,
    C: fmt::Debug,
{
    payload: Option<P>,
    name: String,
    /// The function executed after the trigger condition is met.
    consumer: fn(C),
    /// how many elements are exceeded
    max_len: usize,
    /// The maximum time to wait after an element is saved.
    interval: Option<Duration>,
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

impl<E, C, P> fmt::Debug for Builder<E, C, P>
where
    P: fmt::Debug + Sync + Send,
    E: fmt::Debug + Sync + Send,
    C: fmt::Debug + Sync + Send,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name {}", self.name)
    }
}

impl<E, C, P> Builder<E, C, P>
where
    P: fmt::Debug + Sync + Send,
    E: fmt::Debug + Sync + Send,
    C: fmt::Debug + Sync + Send,
{
    /// init
    #[must_use]
    pub fn builder() -> Self {
        Self {
            payload: None,
            name: "anonymous".to_owned(),
            get_len: |_| 1,
            incr_len: |_| {},
            clear_len: |_| {},
            get_container: |_| panic!(),
            accumulator: |_, _| {},
            get_and_clear_container: |_| panic!(),
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

    /// set `get_len`
    pub fn get_len(mut self, get_len: fn(&Option<P>) -> usize) -> Self {
        self.get_len = get_len;
        self
    }

    /// set `incr_len`
    pub fn incr_len(mut self, incr_len: fn(&mut Option<P>)) -> Self {
        self.incr_len = incr_len;
        self
    }
    /// set `incr_len`
    pub fn clear_len(mut self, clear_len: fn(&mut Option<P>)) -> Self {
        self.clear_len = clear_len;
        self
    }

    /// set `get_container`
    pub fn get_container(mut self, get_container: fn(&mut Option<P>) -> &mut C) -> Self {
        self.get_container = get_container;
        self
    }

    /// set `get_and_clear_container`
    pub fn get_and_clear_container(
        mut self,
        get_and_clear_container: fn(&mut Option<P>) -> C,
    ) -> Self {
        self.get_and_clear_container = get_and_clear_container;
        self
    }

    /// set `accumulator`
    pub fn accumulator(mut self, accumulator: fn(&mut C, E)) -> Self {
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

    /// set `interval`
    pub fn payload(mut self, payload: P) -> Self {
        self.payload = Some(payload);
        self
    }

    /// `build`
    pub fn build(self) -> Outer<General<E, C, P>> {
        let (sender, receiver) = sync::channel(10);
        let general = General {
            name: self.name,
            locker: RwLock::new(Locker {
                get_len: self.get_len,
                incr_len: self.incr_len,
                clear_len: self.clear_len,
                get_container: self.get_container,
                get_and_clear_container: self.get_and_clear_container,
                accumulator: self.accumulator,
                clock: false,
                payload: self.payload,
            }),
            consumer: self.consumer,
            max_len: self.max_len,
            interval: self.interval,
            sender: Mutex::new(sender),
            receiver: Mutex::new(receiver),
        };
        if self.interval.is_some() {
            lifetime_thread::async_spawn(general, |inner| async move {
                while let Some(g) = inner.get() {
                    g.listen_clock_trigger().await
                }
            })
        } else {
            lifetime_thread::spawn(general, |_| {})
        }
    }
}
