# backtracer

A library for acquiring backtraces at runtime for Rust no-std environments.
If you are not in a no-std environment, you probably want to use 
https://github.com/alexcrichton/backtrace-rs instead.

## Install

```toml
[dependencies]
backtracer = "0.0.1"
```

```rust
extern crate backtracer;
```

## Usage

Use the `trace` and `resolve` functions directly.

```rust
extern crate backtracer;

fn main() {
    backtracer::trace(|frame| {
        let ip = frame.ip();
        let symbol_address = frame.symbol_address();

        // Resolve this instruction pointer to a symbol name
        backtracer::resolve(ip, |symbol| {
            if let Some(name) = symbol.name() {
                // ...
            }
            if let Some(filename) = symbol.filename() {
                // ...
            }
        });

        true // keep going to the next frame
    });
}
```

## Platform Support

This should work on any platform with minimal implementation effort.