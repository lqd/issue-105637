# ThinLTO dylib issue 105637 reproduction

Interesting things on `nightly-2022-12-12` (which still has `rustc_driver` built with ThinLTO on linux):
1. the wrong panic hook is to be called
2. symbols are duplicated with `-Zdylib-lto`, some of libstd's static, like the mutable
   `std::panicking::HOOK` storing the current panic hook.

Duplicates the structure between rustc's main binary wrapper and `rustc_driver`'s dylib containing the actual code, to reproduce [issue #105637](https://github.com/rust-lang/rust/issues/105637).

### Example without ThinLTO on dylib
```
cargo clean && RUSTFLAGS="-Cprefer-dynamic" cargo run -q
```

Output:

```
rustc_main - updating panic hook
rustc_driver - DEFAULT_HOOK forced, installing new panic hook
Compiling, about to panic.
rustc_driver - panic hook executing
```

rustc_driver's panic hook is executed.

Looking at the dylib panic hook symbols, it's undefined:
```
nm ./target/debug/librustc_driver.so | rustfilt | grep "std::panicking::HOOK"
                 U std::panicking::HOOK
```

### Example with ThinLTO
```
cargo clean && RUSTFLAGS="-Cprefer-dynamic -Zdylib-lto -Cembed-bitcode -Clto=thin" cargo run -q
```

Output:

```
rustc_main - updating panic hook
rustc_driver - DEFAULT_HOOK forced, installing new panic hook
Compiling, about to panic.
rustc_main - panic hook executing
```

rustc_main's panic hook is executed.

Looking at the dylib panic hook symbols, it's a local BSS symbol:
```
nm ./target/debug/librustc_driver.so | rustfilt | grep "std::panicking::HOOK"
00000000000e4118 b std::panicking::HOOK
```
