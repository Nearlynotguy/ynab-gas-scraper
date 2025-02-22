use clap::Parser;

#[derive(Parser)]
struct Cli {
    month: Option<String>,
    year: Option<String>,
}

fn main() {
    let args = Cli::parse();

    run(&args);
}

fn run(args: &Cli) {}

#[cfg(test)]
mod test {
    use super::*;
    pub trait TestCliValues {
        fn empty() -> Self;
        fn default() -> Self;
    }

    impl TestCliValues for Cli {
        fn empty() -> Self {
            Self {
                month: None,
                year: None,
            }
        }
        fn default() -> Self {
            Self {
                month: Some("JAN".to_string()),
                year: Some("2024".to_string()),
            }
        }
    }

    #[test]
    fn run_with_no_input() {
        let empty_test_input = Cli::empty();
        run(&empty_test_input);
    }

    #[test]
    fn run_with_default_input() {
        let default_test_input = Cli::default();
        run(&default_test_input);
    }
}
