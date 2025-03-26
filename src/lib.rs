fn get_date(date: Option<&str>) -> Option<(u16, u16)> {
    date?
        .split_once('-')
        .map(|d| (d.0.parse::<u16>().ok(), d.1.parse::<u16>().ok()))
        .and_then(|f| match f {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        })
}

#[cfg(test)]
mod get_date_tests {
    use super::*;

    fn check(input_range: Option<&str>, expected_result: Option<(u16, u16)>) {
        let actual_result = get_date(input_range);
        assert_eq!(actual_result, expected_result)
    }

    #[test]
    fn get_date_march_2024() {
        check(Some(&String::from("2024-03")), Some((2024, 3)))
    }

    #[test]
    fn get_date_empty_input() {
        check(None, None)
    }

    #[test]
    fn get_date_empty_year() {
        check(Some(&String::from("3")), None)
    }

    #[test]
    fn get_date_empty_year_hypen() {
        check(Some(&String::from("-3")), None)
    }

    #[test]
    fn get_date_empty_month() {
        check(Some(&String::from("2024")), None)
    }

    #[test]
    fn get_date_empty_month_hypen() {
        check(Some(&String::from("2024-")), None)
    }

    #[test]
    fn get_date_foo() {
        check(Some(&String::from("foo")), None)
    }
}
