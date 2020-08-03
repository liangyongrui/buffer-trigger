#[macro_use]
extern crate lazy_static;
use async_std::task;
use buffer_trigger::{
    AsyncBufferTrigger, AsyncBufferTriggerBuilder, BufferTrigger, SimpleBufferTrigger,
    SimpleBufferTriggerBuilder,
};
use log::LevelFilter;
use std::{sync, thread, time::Duration};

lazy_static! {
    static ref SIMPLE_BUFFER_TRIGGER: SimpleBufferTrigger<i32, Vec<i32>> =
        SimpleBufferTriggerBuilder::<i32, Vec<i32>>::builder(Vec::default)
            .name("test".to_owned())
            .accumulator(|c, e| c.push(e))
            .consumer(|c| log::info!("{:?}", c))
            .max_len(3)
            .interval(Duration::from_millis(500))
            .build();
}
static START: sync::Once = sync::Once::new();

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

    SIMPLE_BUFFER_TRIGGER.push(1);
    SIMPLE_BUFFER_TRIGGER.push(2);
    SIMPLE_BUFFER_TRIGGER.push(3);
    SIMPLE_BUFFER_TRIGGER.push(4);
    SIMPLE_BUFFER_TRIGGER.push(5);

    thread::sleep(Duration::from_secs(5));
}

lazy_static! {
    static ref ASYNC_BUFFER_TRIGGER: AsyncBufferTrigger<i32, Vec<i32>> =
        AsyncBufferTriggerBuilder::<i32, Vec<i32>>::builder(Vec::default)
            .name("test".to_owned())
            .accumulator(|c, e| c.push(e))
            .consumer(|c| log::info!("{:?}", c))
            .max_len(15)
            .interval(Duration::from_millis(500))
            .build();
}
static ASYNC_START: sync::Once = sync::Once::new();

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
