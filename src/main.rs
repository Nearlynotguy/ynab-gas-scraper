use std::env;
use ynab_gas_scraper::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    run(&args);
}
