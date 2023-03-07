use std::collections::HashMap;

use magnus::module::kernel;
use magnus::value::Qnil;
use magnus::{define_module, function, prelude::*, Error, RArray};

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("HelloRust")?;
    module.define_singleton_method("hello", function!(hello, 1))?;
    module.define_singleton_method("rust_fib", function!(fib, 1))?;
    module.define_singleton_method("rust_parse_csv", function!(csv_parse, 1))?;
    module.define_singleton_method("rust_count_employees", function!(count_employees, 1))?;

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

fn csv_parse(path: String) -> Result<RArray, Error> {
    let csv = RArray::new();
    let mut reader = csv::Reader::from_path(path).unwrap();

    for result in reader.deserialize() {
        let record: HashMap<String, String> = result.unwrap();
        csv.push(record)?;
    }

    Ok(csv)
}

fn count_employees(path: String) -> usize {
    let mut count = 0;
    let mut reader = csv::Reader::from_path(path).unwrap();

    for result in reader.records() {
        let record = result.unwrap();
        let employees: usize = record.iter().nth_back(0).unwrap().parse().unwrap();

        count += employees;
    }

    count
}
