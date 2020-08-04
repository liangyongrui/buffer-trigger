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

## Introduction

A data collection trigger based on the maximum number and refresh time.

scenes to be used:

- Aggregate logs, output regularly and quantitatively.
- Aggregate large amounts of MQ data and merge processing.
- For a large number of update requests, you can update the cache first, and then merge and refresh the db.
- ...All operations that require aggregation, throttling, etc. can be used.

## Basic usage

more see [tests](/tests)

```rust
#[macro_use]
extern crate lazy_static;
use buffer_trigger::{
    self, buffer_trigger_sync, buffer_trigger_sync::BufferTrigger,
};
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
```

output:

```text
[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
[15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29]
[30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44]
[45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59]
[60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74]
[75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89]
[90, 91, 92, 93, 94, 95, 96, 97, 98, 99]
```

## Features

This project is still under development. The following features with the check marks are supported.

If you are concerned about an unimplemented feature, please tell me and I will finish writing it ASAP.

- [x] Trigger timing based on quantity
- [x] Trigger based on delay timing (each element can be stored in the container for the maximum time)
- [ ] Different runtime
  - [x] sync (Multithreading)
  - [x] async-std
  - [ ] tokio
- [ ] Multiple type versions
  - [x] general (You can use it to implement remote/local services, such as redis.)
  - [x] simple (local service)
  - [ ] reids (remote service demo)

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions
