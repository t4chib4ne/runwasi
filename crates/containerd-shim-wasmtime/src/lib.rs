mod http_proxy;
pub mod instance;

pub use instance::WasmtimeShim;

mod otel;

#[cfg(unix)]
#[cfg(test)]
#[path = "tests.rs"]
mod wasmtime_tests;
