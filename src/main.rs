use clap::Parser;

#[derive(Parser)]
struct Cli {
    month: Option<String>,
}

fn main() {
    let args = Cli::parse();

    run(&args);
}

fn run(args: &Cli) {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn run_with_no_input() {
        let empty_test_input = Cli {
            month: None,
            // year: None,
        };
        run(&empty_test_input);
    }

    #[test]
    fn run_with_default_input() {
        let default_test_input = Cli {
            month: Some("JAN".to_string()),
            // year: Some("2024".to_string()),
        };
        run(&default_test_input);
    }
}
