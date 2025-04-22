use clap::Parser;
use ynab_gas_scraper::cli::get_date;

#[derive(Parser)]
#[command(about)]
struct Cli {
    #[arg(short = 's', long)]
    start: Option<String>,

    #[arg(short = 'e', long)]
    end: Option<String>,
}

fn main() {
    let args = Cli::parse();

    run(&args);
}

fn run(args: &Cli) {
    let start_date = get_date(&args.start);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn run_with_no_input() {
        let empty_test_input = Cli {
            start: None,
            end: None,
        };
        run(&empty_test_input);
    }

    // #[test]
    // fn run_with_default_input() {
    //     let default_test_input = Cli {
    //         start: Some("JAN".to_string()),
    //     };
    //     run(&default_test_input);
    // }
}
