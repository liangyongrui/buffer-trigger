use super::general::{self, General};
use std::{fmt, mem, time::Duration};
#[derive(Debug)]
struct Payload<C>
where
    C: fmt::Debug,
{
    len: usize,
    container: C,
    defalut_container: fn() -> C,
}

#[derive(Debug)]
pub struct Simple<E, C>
where
    E: fmt::Debug + Sync + Send,
    C: fmt::Debug + Sync + Send,
{
    general: General<E, C, Payload<C>>,
}

impl<E, C> Simple<E, C>
where
    E: fmt::Debug + Sync + Send,
    C: fmt::Debug + Sync + Send,
{
    pub async fn is_empty(&self) -> bool {
        self.general.is_empty().await
    }
    pub async fn len(&self) -> usize {
        self.general.len().await
    }
    pub async fn push(&self, value: E) {
        self.general.push(value).await
    }
    pub async fn trigger(&self) {
        self.general.trigger().await
    }
    pub async fn listen_clock_trigger(&self) {
        self.general.listen_clock_trigger().await
    }
}

pub struct Builder<E, C>
where
    E: fmt::Debug,
    C: fmt::Debug,
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
    #[must_use]
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
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
    #[must_use]
    pub fn max_len(mut self, max_len: usize) -> Self {
        self.max_len = max_len;
        self
    }

    /// set `interval`
    #[must_use]
    pub fn interval(mut self, interval: Duration) -> Self {
        self.interval = Some(interval);
        self
    }

    /// `build`
    #[must_use]
    pub fn build(self) -> Simple<E, C> {
        let payload = Payload {
            container: (self.defalut_container)(),
            defalut_container: self.defalut_container,
            len: 0,
        };

        let mut general = general::builder::Builder::builder();
        if let Some(t) = self.interval {
            general = general.interval(t);
        }
        let general = general
            .consumer(self.consumer)
            .max_len(self.max_len)
            .payload(payload)
            .get_len(|p| p.as_ref().unwrap().len)
            .incr_len(|p| p.as_mut().unwrap().len += 1)
            .clear_len(|p| p.as_mut().unwrap().len = 0)
            .get_container(|p| &mut p.as_mut().unwrap().container)
            .get_and_clear_container(|p| {
                let mut new_container = (p.as_ref().unwrap().defalut_container)();
                mem::swap(&mut new_container, &mut p.as_mut().unwrap().container);
                new_container
            })
            .accumulator(self.accumulator)
            .build();

        Simple { general }
    }
}
