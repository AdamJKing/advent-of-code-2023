use std::fs;

pub(crate) fn day_one() {
    let binding =
        fs::read_to_string("data/day_one.txt").expect("Could not load data file day_one.txt");
    let mut day_one = binding.lines();
    println!("{}", day_one_impl(&mut day_one));
}

fn day_one_impl<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> u32 {
    input.into_iter().filter_map(calibration_value).sum()
}

fn calibration_value(input: &str) -> Option<u32> {
    // let numbers = input.chars().filter(|c| c.is_numeric());
    let mut numbers = input.chars().filter_map(|c| c.to_digit(10));

    let first = numbers.next()?;
    let last = numbers.next_back().unwrap_or(first);

    Some((first * 10) + last)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::{calibration_value, day_one_impl};

    #[test]
    fn spec() {
        let input = indoc! {"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "};

        assert_eq!(day_one_impl(&mut input.lines()), 142);
    }

    #[test]
    fn calibration_value_spec() {
        assert_eq!(calibration_value("12"), Some(12));
        assert_eq!(calibration_value("1a2"), Some(12));
        assert_eq!(calibration_value("1a3n2"), Some(12));
        assert_eq!(calibration_value(""), None);
        assert_eq!(calibration_value("abc"), None);
        assert_eq!(calibration_value("1abc"), Some(11));
    }
}
