use crate::BufferTrigger;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::{fmt, mem, time::Duration};
/// 简单的 `BufferTrigger`
/// 自己设置容器在当前服务存储
pub struct SimpleBufferTrigger<E, C>
where
    E: fmt::Debug,
{
    /// 名字
    name: String,
    /// 容器的默认生成函数
    defalut_container: fn() -> C,
    /// 容器元素个数
    len: usize,
    /// 缓存
    container: C,
    /// 聚合函数
    accumulator: fn(&mut C, E) -> (),
    /// 需要执行的方法
    consumer: fn(C) -> (),
    /// 超过多少元素后触发
    max_len: usize,
    /// 是否有闹钟
    clock: bool,
    /// 一个元素被保存后最多等待的时长
    interval: Option<Duration>,
    // 通知触发的mpsc
    channel: (Sender<()>, Receiver<()>),
}
impl<E, C> fmt::Debug for SimpleBufferTrigger<E, C>
where
    E: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name {}", self.name)
    }
}
impl<E, C> BufferTrigger<E> for SimpleBufferTrigger<E, C>
where
    E: fmt::Debug,
{
    fn len(&self) -> usize {
        self.len
    }
    fn push(&mut self, value: E) {
        (self.accumulator)(&mut self.container, value);
        let clock = self.clock;
        self.len += 1;
        if !clock {
            if let Some(dur) = self.interval {
                self.clock = true;
                let sender = self.channel.0.clone();
                let _ = thread::spawn(move || {
                    thread::sleep(dur);
                    if let Err(e) = sender.send(()) {
                        log::error!("auto clock trigger error {}", e);
                    }
                });
            }
        }
        if self.len() >= self.max_len {
            self.trigger()
        }
    }

    fn trigger(&mut self) {
        if !self.is_empty() {
            self.len = 0;
            let mut new_cahce = (self.defalut_container)();
            mem::swap(&mut new_cahce, &mut self.container);
            self.clock = false;
            (self.consumer)(new_cahce);
        }
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<E, C> SimpleBufferTrigger<E, C>
where
    E: fmt::Debug,
{
    /// 监听时钟触发
    pub fn listen_clock_trigger(&mut self) {
        log::info!("{:?} listen_clock_trigger", self);
        while self.channel.1.recv().is_ok() {
            if self.clock {
                self.trigger();
            }
        }
    }
}

impl<E, C> Drop for SimpleBufferTrigger<E, C>
where
    E: fmt::Debug,
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
    /// name
    name: String,
    /// 缓存
    defalut_container: fn() -> C,
    /// 聚合函数
    accumulator: fn(&mut C, E),
    /// 需要执行的方法
    consumer: fn(C),
    /// 超过多少元素后触发
    max_len: usize,
    /// 一个元素被保存后最多等待的时长
    interval: Option<Duration>,
}

impl<E, C> fmt::Debug for Builder<E, C>
where
    E: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name {}", self.name)
    }
}
impl<E, C> Builder<E, C>
where
    E: fmt::Debug,
{
    /// init
    pub fn builder() -> Self {
        Self {
            name: "anonymous".to_owned(),
            defalut_container: || panic!(),
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
    /// set `default_container`
    pub fn default_container(mut self, defalut_container: fn() -> C) -> Self {
        self.defalut_container = defalut_container;
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
        SimpleBufferTrigger {
            name: self.name,
            len: 0,
            defalut_container: self.defalut_container,
            container: (self.defalut_container)(),
            accumulator: self.accumulator,
            consumer: self.consumer,
            max_len: self.max_len,
            clock: false,
            interval: self.interval,
            channel: mpsc::channel(),
        }
    }
}
