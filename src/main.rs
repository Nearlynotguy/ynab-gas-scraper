use clap::Parser;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    run(&args);
}

fn run<T>(args: T) {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_with_no_input() {
        let empty_test_input: &[String] = &[];
        run(empty_test_input);
    }

    #[test]
    fn run_with_arbitrary_input() {
        let test_input = 0;
        run(test_input);
    }

    #[test]
    fn run_with_string() {
        let test_input = &"hello".to_string();
        run(test_input);
    }
}
