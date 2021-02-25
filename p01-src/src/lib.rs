pub mod closest;
pub mod geometry;
pub mod parse;
pub mod quick_select;

/// Bails out an error, printing a message to stdout, and then quit the process.
#[macro_export]
macro_rules! bail_error {
    ($exit_code : expr) => {{
        use std::process;
        println!("{}", "Incorrectly formatted input");
        process::exit($exit_code)
    }};
}
