<h1 align="center">Buffer Trigger</h1>
<div align="center">
 <strong>
  一个基于最大数量和刷新时间的数据收集触发器
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

一个基于数量和时间的数据收集触发器

使用场景：

- 聚合日志，定时定量输出
- 聚合 MQ 的大量数据，合并处理
- 大量更新请求时，可以先更新 cache，再合并刷 db
- ...所有需要聚合、节流等操作都可以用

## Basic usage

see [tests](/tests)

```rust
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
```

output:

```text
[1, 2, 3]
[4, 5]
```

## Features

This project is still under development. The following features with the check marks are supported.

If you are concerned about an unimplemented feature, please tell me and I will finish writing it ASAP.

- [x] 根据数量定时触发
- [x] 根据延迟定时触发（每个元素最多在容器中保存的时长）
- [ ] 多种类型容器
  - [x] 本地容器存储
  - [ ] 远程容器存储(redis)
- [ ] 可以指定 runtime 的异步版本
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
