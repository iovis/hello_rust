use magnus::module::kernel;
use magnus::value::Qnil;
use magnus::{define_module, function, prelude::*, Error};

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("HelloRust")?;
    module.define_singleton_method("hello", function!(hello, 1))?;
    module.define_singleton_method("rust_fib", magnus::function!(fib, 1))?;

    Ok(())
}

// Inefficient on purpose to check differences in performance
fn fib(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

#[allow(dead_code)]
fn puts(val: &str) -> Qnil {
    kernel().funcall("puts", (val,)).unwrap()
}

#[allow(clippy::needless_pass_by_value)]
// Magnus needs to pass Strings instead of &str
fn hello(subject: String) -> String {
    format!("Hello from Rust, {subject}!")
}
