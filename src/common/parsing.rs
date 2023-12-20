use std::{fmt::Debug, io::BufRead, str::FromStr};

/// Reads the input to a line iterator.
///
/// ```
/// # use adventofcode::common::parsing::lines_iter;
/// let mut input = "aaa\nbbb".as_bytes();
/// let mut lines = lines_iter(&mut input);
/// # assert_eq!(lines.next(), Some("aaa".to_string()));
/// # assert_eq!(lines.next(), Some("bbb".to_string()));
/// # assert_eq!(lines.next(), None);
/// ```
pub fn lines_iter(input: &mut dyn BufRead) -> impl Iterator<Item = String> + '_ {
    input.lines().map(|l| l.unwrap())
}

/// Reads the input to a string vector.
///
/// ```
/// # use adventofcode::common::parsing::lines_vec;
/// let lines = lines_vec(&mut "aaa\nbbb".as_bytes());
/// # assert_eq!(lines, vec!["aaa".to_string(), "bbb".to_string()]);
/// ```
pub fn lines_vec(input: &mut dyn BufRead) -> Vec<String> {
    lines_iter(input).collect()
}

/// Parse all lines of the input to a `Vec<T>`.
///
/// ```
/// # use adventofcode::common::parsing::parse_lines_vec;
/// let lines: Vec<u8> = parse_lines_vec(&mut "1\n2\n3".as_bytes());
/// # assert_eq!(lines, vec![1, 2, 3]);
/// ```
pub fn parse_lines_vec<T>(input: &mut dyn BufRead) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    lines_iter(input)
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}
