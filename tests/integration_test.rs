#[macro_use]
extern crate lazy_static;
use buffer_trigger::{BufferTrigger, SimpleBufferTrigger, SimpleBufferTriggerBuilder};
use log::LevelFilter;
use std::{sync::Mutex, thread, time::Duration};

lazy_static! {
    static ref BUFFER_TRIGGER: Mutex<SimpleBufferTrigger<i32, Vec<i32>>> = Mutex::new(
        SimpleBufferTriggerBuilder::<i32, Vec<i32>>::builder()
            .name("test".to_owned())
            .default_container(Vec::default)
            .accumulator(|c, e| c.push(e))
            .consumer(|c| println!("{:?}", c))
            .max_len(2)
            .interval(Duration::from_millis(500))
            .build()
    );
}
#[test]
fn it_works() {
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(LevelFilter::Debug)
        .try_init();

    thread::spawn(|| {
        BUFFER_TRIGGER.lock().unwrap().listen_clock_trigger();
    });

    BUFFER_TRIGGER.lock().unwrap().push(1);
    BUFFER_TRIGGER.lock().unwrap().push(2);
    BUFFER_TRIGGER.lock().unwrap().push(3);
    BUFFER_TRIGGER.lock().unwrap().push(4);
    BUFFER_TRIGGER.lock().unwrap().push(5);

    thread::sleep(Duration::from_secs(5));
}
