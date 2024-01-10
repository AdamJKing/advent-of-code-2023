use itertools::{Itertools, PeekingNext};
use lazy_static::lazy_static;
use std::{collections::HashMap, fs, iter::Peekable};

pub(crate) fn day_one() {
    let binding =
        fs::read_to_string("data/day_one.txt").expect("Could not load data file day_one.txt");
    let mut day_one = binding.lines();
    println!("Day One Part 1: {}", day_one_impl_pt1(&mut day_one));
    println!("Day One Part 1: {}", day_one_impl_pt2(&mut day_one));
}

fn day_one_impl_pt1<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> u32 {
    input.into_iter().filter_map(calibration_value).sum()
}

fn day_one_impl_pt2<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> u32 {
    input
        .into_iter()
        .filter_map(calibration_value_improved)
        .sum()
}

fn calibration_value(input: &str) -> Option<u32> {
    let mut numbers = input.chars().filter_map(|c| c.to_digit(10));

    let first = numbers.next()?;
    let last = numbers.next_back().unwrap_or(first);

    Some((first * 10) + last)
}

lazy_static! {
    static ref LOOKUP: HashMap<String, u32> = HashMap::from([
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
    ]);
}

fn capture_number_from_text<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> Option<u32> {
    let mut token = String::new();

    iter.find_map(|item| {
        token.push(item);

        if !LOOKUP.keys().any(|key| key.starts_with(&token.clone())) {
            token = "".to_owned();
        }

        LOOKUP.get(&token as &str).copied()
    })
}

fn capture_number<I: Iterator<Item = char>>(peeked: &mut Peekable<I>) -> Option<u32> {
    peeked
        .peeking_next(|ch| ch.is_ascii_digit())
        .and_then(|ch| ch.to_digit(10))
        .or_else(|| capture_number_from_text(peeked))
}

fn capture_numbers(input: &str) -> Vec<u32> {
    input
        .chars()
        .peekable()
        .batching(capture_number)
        .collect_vec()
}

fn calibration_value_improved(input: &str) -> Option<u32> {
    let numbers = capture_numbers(input);
    let first = numbers.first()?;
    let last = numbers.last().unwrap_or(first);

    Some((first * 10) + last)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use crate::day_one::{
        calibration_value, calibration_value_improved, capture_number, capture_number_from_text,
        capture_numbers, day_one_impl_pt1, day_one_impl_pt2,
    };

    #[test]
    fn spec_pt1() {
        let input = indoc! {"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "};

        assert_eq!(day_one_impl_pt1(&mut input.lines()), 142);
    }

    #[test]
    fn spec_pt2() {
        let input = indoc! {"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "};

        assert_eq!(day_one_impl_pt2(&mut input.lines()), 281);
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

    #[test]
    fn calibration_value_improved_spec() {
        assert_eq!(calibration_value_improved("two1nine"), Some(29));
        assert_eq!(calibration_value_improved("eightwothree"), Some(83));
        assert_eq!(calibration_value_improved("abcone2threexyz"), Some(13));
        assert_eq!(calibration_value_improved("xtwone3four"), Some(24));
        assert_eq!(calibration_value_improved("4nineeightseven2"), Some(42));
        assert_eq!(calibration_value_improved("zoneight234"), Some(14));
        assert_eq!(calibration_value_improved("7pqrstsixteen"), Some(76));
    }

    #[test]
    fn capture_number_from_text_spec() {
        assert_eq!(
            capture_number_from_text(&mut "one".chars().peekable()),
            Some(1)
        );
        assert_eq!(
            capture_number_from_text(&mut "seven".chars().peekable()),
            Some(7)
        );

        let mut iter = "oneabc".chars().peekable();
        assert_eq!(capture_number_from_text(&mut iter), Some(1));
        assert_eq!(iter.next(), Some('a'));

        assert_eq!(
            capture_number_from_text(&mut "wotwo".chars().peekable()),
            Some(2)
        );
    }

    #[test]
    fn capture_number_spec() {
        assert_eq!(capture_number(&mut "one".chars().peekable()), Some(1));
        assert_eq!(capture_number(&mut "1".chars().peekable()), Some(1));

        let mut iter = "1one".chars().peekable();
        assert_eq!(capture_number(&mut iter), Some(1));
        assert_eq!(iter.peek().is_some(), true);
    }

    #[test]
    fn capture_numbers_spec() {
        assert_eq!(capture_numbers("two"), vec![2]);
        assert_eq!(capture_numbers("1nine"), vec![1, 9]);
        assert_eq!(capture_numbers("two1nine"), vec![2, 1, 9]);

        assert_eq!(capture_numbers("eightwothree"), vec![8, 2, 3]);
    }
}
