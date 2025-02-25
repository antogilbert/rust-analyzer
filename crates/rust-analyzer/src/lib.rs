//! Implementation of the LSP for rust-analyzer.
//!
//! This crate takes Rust-specific analysis results from ide and translates
//! into LSP types.
//!
//! It also is the root of all state. `world` module defines the bulk of the
//! state, and `main_loop` module defines the rules for modifying it.
//!
//! The `cli` submodule implements some batch-processing analysis, primarily as
//! a debugging aid.

pub mod cli;

#[allow(unused)]
macro_rules! eprintln {
    ($($tt:tt)*) => { stdx::eprintln!($($tt)*) };
}

mod caps;
mod cargo_target_spec;
mod diagnostics;
mod diff;
mod dispatch;
mod from_proto;
mod global_state;
mod handlers;
mod line_index;
mod lsp_utils;
mod main_loop;
mod markdown;
mod mem_docs;
mod op_queue;
mod reload;
mod semantic_tokens;
mod task_pool;
mod to_proto;
mod version;

pub mod config;
pub mod lsp_ext;

#[cfg(test)]
mod integrated_benchmarks;

use std::fmt;

use serde::de::DeserializeOwned;

pub use crate::{caps::server_capabilities, main_loop::main_loop, version::version};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub fn from_json<T: DeserializeOwned>(what: &'static str, json: &serde_json::Value) -> Result<T> {
    let res = serde_json::from_value(json.clone())
        .map_err(|e| format!("Failed to deserialize {}: {}; {}", what, e, json))?;
    Ok(res)
}

#[derive(Debug)]
struct LspError {
    code: i32,
    message: String,
}

impl LspError {
    fn new(code: i32, message: String) -> LspError {
        LspError { code, message }
    }
}

impl fmt::Display for LspError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Language Server request failed with {}. ({})", self.code, self.message)
    }
}

impl std::error::Error for LspError {}
