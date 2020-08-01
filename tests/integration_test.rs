#[macro_use]
extern crate lazy_static;
use buffer_trigger::{BufferTrigger, SimpleBufferTrigger, SimpleBufferTriggerBuilder};
use log::LevelFilter;
use std::{thread, time::Duration};

lazy_static! {
    static ref BUFFER_TRIGGER: SimpleBufferTrigger<i32, Vec<i32>> =
        SimpleBufferTriggerBuilder::<i32, Vec<i32>>::builder(Vec::default)
            .name("test".to_owned())
            .accumulator(|c, e| c.push(e))
            .consumer(|c| log::info!("{:?}", c))
            .max_len(3)
            .interval(Duration::from_millis(500))
            .build();
}

#[test]
fn it_works() {
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(LevelFilter::Debug)
        .try_init();

    thread::spawn(|| {
        BUFFER_TRIGGER.listen_clock_trigger();
    });

    BUFFER_TRIGGER.push(1);
    BUFFER_TRIGGER.push(2);
    BUFFER_TRIGGER.push(3);
    BUFFER_TRIGGER.push(4);
    BUFFER_TRIGGER.push(5);

    thread::sleep(Duration::from_secs(5));
}
