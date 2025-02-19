use std::process;

struct Request {
    all_dates: bool,
    month_range: String,
    year_range: String,
}

impl Request {
    fn build_all() -> Request {
        Request {
            all_dates: true,
            month_range: String::new(),
            year_range: String::new(),
        }
    }

    fn build_month(month: &String, year: &String) -> Request {
        Request {
            all_dates: false,
            month_range: month.clone(),
            year_range: year.clone(),
        }
    }
}

pub fn run(args: &[String]) {
    let month = &args[1];
    let year = &args[2];

    println!("Month: {}", month);
    println!("Year: {}", year);

    process::exit(0);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn make_request_all() {
        let test_req = Request::build_all();
        assert!(test_req.all_dates)
    }

    #[test]
    fn make_request_month() {
        let test_month: String = "oct".to_string();
        let test_year: String = "2024".to_string();
        let test_req = Request::build_month(&test_month, &test_year);
        assert_eq!(
            (test_req.month_range, test_req.year_range),
            (test_month, test_year)
        );
    }
}
