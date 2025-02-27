fn get_months(month_range: Option<String>) -> Option<(i32, i32)> {
    match month_range {
        Some(month_range) => {
            let mut months = month_range
                .split_whitespace()
                .map(|c| c.parse::<i32>().unwrap());
            Some((months.next().unwrap(), months.next().unwrap_or(12)))
        }
        None => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn check(input_range: Option<String>, expected_result: Option<(i32, i32)>) {
        let actual_result = get_months(input_range);
        assert_eq!(actual_result, expected_result)
    }

    #[test]
    fn get_months_from_mar_to_jun() {
        check(Some(String::from("3 6")), Some((3, 6)))
    }

    #[test]
    fn get_months_empty_input() {
        check(None, None)
    }

    #[test]
    fn get_months_no_end() {
        check(Some(String::from("3")), Some((3, 12)))
    }
}
