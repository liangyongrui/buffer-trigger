<h1 align="center">Buffer Trigger</h1>
<div align="center">
 <strong>
    A data collection trigger based on the maximum number and refresh time.
 </strong>
</div>
<br />
<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/buffer-trigger">
    <img src="https://img.shields.io/crates/v/buffer-trigger.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/buffer-trigger">
    <img src="https://img.shields.io/crates/d/buffer-trigger.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/buffer-trigger">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <!-- ci -->
  <a href="https://docs.rs/buffer-trigger">
    <img src="https://github.com/liangyongrui/buffer-trigger/workflows/Rust/badge.svg"
      alt="ci" />
  </a>
  <!-- coverage -->
  <a href="https://codecov.io/gh/liangyongrui/buffer-trigger">
    <img src="https://codecov.io/gh/liangyongrui/buffer-trigger/branch/master/graph/badge.svg" />
  </a>
</div>

<br/>

## Other language versions

[简体中文](/readme-zh.md)

## Introduction

A data collection trigger based on the maximum number and refresh time.

scenes to be used:

- Aggregate logs, output regularly and quantitatively.
- Aggregate large amounts of MQ data and merge processing.
- For a large number of update requests, you can update the cache first, and then merge and refresh the db.
- ...All operations that require aggregation, throttling, etc. can be used.

## Basic usage

see [tests](/tests)

```rust
#[macro_use]
extern crate lazy_static;
use buffer_trigger::{BufferTrigger, SimpleBufferTrigger, SimpleBufferTriggerBuilder};
use log::LevelFilter;
use std::{sync, thread, time::Duration};

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
static START: sync::Once = sync::Once::new();

#[test]
fn it_works() {
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(LevelFilter::Debug)
        .try_init();

    START.call_once(|| {
        thread::spawn(|| {
            BUFFER_TRIGGER.listen_clock_trigger();
        });
    });

    BUFFER_TRIGGER.push(1);
    BUFFER_TRIGGER.push(2);
    BUFFER_TRIGGER.push(3);
    BUFFER_TRIGGER.push(4);
    BUFFER_TRIGGER.push(5);

    thread::sleep(Duration::from_secs(5));
}
```

output:

```text
[1, 2, 3]
[4, 5]
```

## Features

This project is still under development. The following features with the check marks are supported.

If you are concerned about an unimplemented feature, please tell me and I will finish writing it ASAP.

- [x] Trigger timing based on quantity
- [x] Trigger based on delay timing (each element can be stored in the container for the maximum time)
- [ ] Various types of containers
  - [x] Local container storage
  - [ ] Remote Container Storage (redis)
- [ ] You can specify the asynchronous version of runtime
  - [ ] async-std
  - [ ] tokio

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions
