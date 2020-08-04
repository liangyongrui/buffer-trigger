#[macro_use]
extern crate lazy_static;
use async_std::task;
use buffer_trigger::{
    self, buffer_trigger_async, buffer_trigger_sync, buffer_trigger_sync::BufferTrigger,
};
use log::LevelFilter;
use std::{sync::Once, thread, time::Duration};

lazy_static! {
    static ref SIMPLE_BUFFER_TRIGGER: buffer_trigger_sync::Simple<i32, Vec<i32>> =
        buffer_trigger_sync::SimpleBuilder::builder(Vec::default)
            .name("test".to_owned())
            .accumulator(|c, e| c.push(e))
            .consumer(|c| log::info!("{:?}", c))
            .max_len(15)
            .interval(Duration::from_millis(500))
            .build();
}
static START: Once = Once::new();

#[test]
fn simple_test() {
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(LevelFilter::Debug)
        .try_init();

    START.call_once(|| {
        thread::spawn(|| {
            SIMPLE_BUFFER_TRIGGER.listen_clock_trigger();
        });
    });

    for i in 0..100 {
        SIMPLE_BUFFER_TRIGGER.push(i);
    }

    thread::sleep(Duration::from_secs(5));
}

lazy_static! {
    static ref ASYNC_BUFFER_TRIGGER: buffer_trigger_async::Simple<i32, Vec<i32>> =
        buffer_trigger_async::SimpleBuilder::builder(Vec::default)
            .name("test".to_owned())
            .accumulator(|c, e| c.push(e))
            .consumer(|c| log::info!("{:?}", c))
            .max_len(15)
            .interval(Duration::from_millis(500))
            .build();
}
static ASYNC_START: Once = Once::new();

#[async_std::test]
async fn async_test() {
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(LevelFilter::Debug)
        .try_init();

    ASYNC_START.call_once(|| {
        task::spawn(async {
            ASYNC_BUFFER_TRIGGER.listen_clock_trigger().await;
        });
    });

    for i in 0..100 {
        ASYNC_BUFFER_TRIGGER.push(i).await;
    }

    task::sleep(Duration::from_secs(5)).await;
}
