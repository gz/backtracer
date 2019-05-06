//! A library for acquiring a backtrace at runtime
#![no_std]

extern crate alloc;
extern crate x86;

#[macro_use]
extern crate cfg_if;
extern crate rustc_demangle;
extern crate elfloader;

pub use backtrace::{trace, trace_from, EntryPoint, Frame};
mod backtrace;

pub use symbolize::{resolve, Symbol, SymbolName};
mod symbolize;

#[allow(dead_code)]
struct Bomb {
    enabled: bool,
}

#[allow(dead_code)]
impl Drop for Bomb {
    fn drop(&mut self) {
        if self.enabled {
            panic!("cannot panic during the backtrace function");
        }
    }
}
