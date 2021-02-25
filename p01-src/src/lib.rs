pub mod closest;
pub mod geometry;
pub mod parse;
pub mod quick_select;

/// Message to use when invalid input is detected
pub const BAD_INPUT: &str = "Incorrectly formatted input";

/// Bails out an error, printing a message to stdout, and then quit the process.
#[macro_export]
macro_rules! bail_error {
    ($error: expr, $exit_code : expr) => {{
        use std::process;
        println!("{}", $error);
        process::exit($exit_code)
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
