//! buffer trigger

#![deny(
    unused_braces,
    missing_docs,
    bare_trait_objects,
    missing_copy_implementations,
    single_use_lifetimes,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    variant_size_differences,
    unsafe_code,
    trivial_casts,
    missing_debug_implementations,
    warnings,
    clippy::all,
    clippy::correctness,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::cargo,
    clippy::pedantic,
    clippy::nursery
)]
#![allow(
    clippy::non_ascii_literal,
    clippy::must_use_candidate,
    clippy::dbg_macro,
    clippy::cargo_common_metadata,
    clippy::multiple_crate_versions,
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::use_self, // 泛型会出问题
    clippy::default_trait_access, // 宏里面使用Default::default
    clippy::used_underscore_binding, // 好像存在误报
    clippy::redundant_pub_crate, // 和unreachable_pub冲突
)]

/// 支持的功能
pub trait BufferTrigger<T> {
    /// 是否没有数据
    fn is_empty(&self) -> bool;

    /// 获取Buffer中元素的个数
    fn len(&self) -> usize;

    /// 添加元素
    fn push(&mut self, value: T);

    /// 手动触发
    /// drop 的时候记得触发
    fn trigger(&mut self);
}

mod simple_buffer_trigger;

pub use simple_buffer_trigger::{Builder as SimpleBufferTriggerBuilder, SimpleBufferTrigger};
