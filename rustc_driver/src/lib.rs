#![feature(once_cell)]
#![feature(panic_update_hook)]

use std::panic;
use std::sync::LazyLock;

static DEFAULT_HOOK: LazyLock<()> = LazyLock::new(|| {
    eprintln!("rustc_driver - DEFAULT_HOOK forced, installing new panic hook");
    panic::update_hook(move |_prev, _info| {
        eprintln!("rustc_driver - panic hook executing");
    });
});

pub fn install_ice_hook() {
    LazyLock::force(&DEFAULT_HOOK);
}

pub fn main() -> ! {
    install_ice_hook();

    let exit_code = catch_with_exit_code(|| run_compiler());

    std::process::exit(exit_code)
}

/// Variant of `catch_fatal_errors` for the `interface::Result` return type
/// that also computes the exit code.
pub fn catch_with_exit_code(f: impl FnOnce() -> Result<(), ()>) -> i32 {
    let result = catch_fatal_errors(f).and_then(|result| result);
    match result {
        Ok(()) => 0,
        Err(_) => 1,
    }
}

/// Runs a closure and catches unwinds triggered by fatal errors.
pub fn catch_fatal_errors<F: FnOnce() -> R, R>(f: F) -> Result<R, ()> {
    panic::catch_unwind(panic::AssertUnwindSafe(f)).map_err(|value| {
        panic::resume_unwind(value);
    })
}

fn run_compiler() -> Result<(), ()> {
    eprintln!("Compiling, about to panic.");
    panic!("ICEing");
}
