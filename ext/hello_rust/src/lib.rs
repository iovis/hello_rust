use magnus::{define_module, function, prelude::*, Error};

fn fib(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn hello(subject: String) -> String {
    format!("Hello from Rust, {subject}!")
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("HelloRust")?;
    module.define_singleton_method("hello", function!(hello, 1))?;
    module.define_singleton_method("rust_fib", magnus::function!(fib, 1))?;

    Ok(())
}
