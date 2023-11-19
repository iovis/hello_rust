use std::collections::HashMap;

use magnus::value::Qnil;
use magnus::{function, prelude::*, RArray, Ruby};
use polars::prelude::*;

type RbResult<T> = Result<T, magnus::Error>;

#[magnus::init]
fn init(ruby: &Ruby) -> RbResult<()> {
    let module = ruby.define_module("HelloRust")?;
    module.define_singleton_method("hello", function!(hello, 1))?;
    module.define_singleton_method("rust_fib", function!(fib, 1))?;
    module.define_singleton_method("rust_parse_csv", function!(csv_parse, 1))?;
    module.define_singleton_method("rust_count_employees", function!(count_employees, 1))?;
    module.define_singleton_method(
        "rust_count_employees_polars",
        function!(count_employees_polars, 1),
    )?;

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
fn puts(ruby: &Ruby, val: &str) -> RbResult<Qnil> {
    ruby.module_kernel().funcall("puts", (val,))
}

#[allow(clippy::needless_pass_by_value)]
// Magnus needs to pass Strings instead of &str
fn hello(subject: String) -> String {
    format!("Hello from Rust, {subject}!")
}

fn csv_parse(path: String) -> RbResult<RArray> {
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

fn count_employees_polars(path: String) -> i64 {
    let df = LazyCsvReader::new(path).finish().unwrap();

    let sum_df = df
        .select([col("Number of employees").sum()])
        .collect()
        .unwrap();

    if let Ok(AnyValue::Int64(sum)) = sum_df[0].get(0) {
        return sum;
    }

    panic!("not an int");
}
