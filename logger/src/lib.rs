//! `hexi_logger` provides a dead simple logger implemenation for the [`log`](https://crates.io/crates/log)
//! crate.
//!
//! - It provides no fancy features at all, and just logs directly to stderr, period.
//! - It has zero dependencies aside from the [`log`](https://crates.io/crates/log) crate itself.
//! - It works with the `wasm32` target and fowards to `console.log`, `debug`, `info`, `warn` and
//!   `error` when appropriate.
//!
//! **Note:** `hexi_logger` should only be used in executables, not libraries. For libraries, you
//!           should only use the [`log`](https://crates.io/crates/log) crate macros (this lets
//!           executables decide the logging implementation).
//!
//! # Examples
//!
//! Add it to your `Cargo.toml` along with the `log` dependency:
//!
//! ```toml
//! [dependencies]
//! log = "0.4.0"
//! env_logger = "0.5.9"
//! ```
//!
//! Initialize the logger as early as possible, then use any of the `log` crate macros:
//!
//! ```
//! #[macro_use]
//! extern crate log;
//! extern crate hexi_logger;
//!
//! fn main() {
//!     hexi_logger::init(hexi_logger::Info).unwrap();
//!
//!     trace!("a trace message");
//!     debug!("a debug message");
//!     info!("a info message");
//!     warn!("a warn message");
//!     error!("a error message");
//! }
//! ```
//!
//! Note that the parameter to `hexi_logger::init` is the maximum log level. That is:
//!
//! | Level                | Prints                                                        |
//! |----------------------|---------------------------------------------------------------|
//! | `hexi_logger::Trace` | All messages.                                                 |
//! | `hexi_logger::Debug` | All messages, except `trace!`.                                |
//! | `hexi_logger::Info`  | All messages, except `trace!` and `debug!`.                   |
//! | `hexi_logger::Warn`  | All messages, except `trace!`, `debug!` and `info!`.          |
//! | `hexi_logger::Error` | All messages, except `trace!`, `debug!`, `info!` and `warn!`. |
//! | `hexi_logger::Off`   | No messages are printed.                                      |
//!

#[cfg_attr(test, macro_use)]
extern crate log;

pub use log::LevelFilter::*;
use log::{LevelFilter, SetLoggerError};

#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;

struct Stderr;

impl log::Log for Stderr {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            eprintln!("{}: {}", record.level(), record.args());

            #[cfg(target_arch = "wasm32")]
            {
                let msg = format!("{}: {}", record.level(), record.args());
                let noLvl = format!("{}", record.args());
                match record.level() {
                    log::Level::Trace => js!(console.log(@{msg})),
                    log::Level::Debug => js!(console.debug(@{noLvl})),
                    log::Level::Info => js!(console.info(@{noLvl})),
                    log::Level::Warn => js!(console.warn(@{noLvl})),
                    log::Level::Error => js!(console.error(@{noLvl})),
                };
            }
        }
    }

    fn flush(&self) {}
}

static LOGGER: Stderr = Stderr {};

/// Initializes the logger to log all messages at max to the specified level.
///
/// This function should only be called once for the entire duration of the
/// program.
pub fn init(max: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_max_level(max);
    log::set_logger(&LOGGER)
}

mod tests {
    #[test]
    fn it_logs() {
        super::init(super::Trace).unwrap();
        trace!("trace");
        debug!("debug");
        info!("info");
        warn!("warn");
        error!("error");
    }
}
