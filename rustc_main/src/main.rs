#![feature(panic_update_hook)]

use std::panic;

fn main() {
    eprintln!("rustc_main - updating panic hook");
    panic::update_hook(move |_prev, _info| {
        eprintln!("rustc_main - panic hook executing",);
    });

    rustc_driver::main();
}
