use std::convert::TryFrom;
use std::ops::Range;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\d*)-(\d*) ([a-z]): ([a-z]*)$").unwrap();
}

pub struct PasswordConstraint {
    range: Range<usize>,
    letter: char,
    password: String,
}

impl PasswordConstraint {
    pub fn basic(&self) -> bool {
        let count = self.password.chars().filter(|c| *c == self.letter).count();
        self.range.start <= count && count <= self.range.end
    }

    pub fn complex(&self) -> bool {
        // Get the 2 characters
        let left = self.password.chars().nth(self.range.start - 1).unwrap();
        let right = self.password.chars().nth(self.range.end - 1).unwrap();

        (left == self.letter) ^ (right == self.letter)
    }
}

impl TryFrom<&str> for PasswordConstraint {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let captures = RE
            .captures(input)
            .ok_or(format!("Input: {} failed the regular expression", input))?;

        if captures.len() != 5 {
            return Err("Failed to find the right number of captures".into());
        }

        // Unwrap is fine here as we validated with the Regex
        let lower = usize::from_str(&captures[1]).unwrap();
        let upper = usize::from_str(&captures[2]).unwrap();
        let letter = &captures[3].chars().next().unwrap();

        let password = String::from(&captures[4]);

        Ok(PasswordConstraint {
            range: (lower..upper),
            letter: *letter,
            password,
        })
    }
}

pub fn read_input() -> Vec<PasswordConstraint> {
    std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split('\n')
        .map(|line| PasswordConstraint::try_from(line).unwrap())
        .collect()
}

pub fn valid_passwords<F>(passwords: &[PasswordConstraint], constraint: F) -> usize
where
    F: Fn(&PasswordConstraint) -> bool,
{
    passwords.iter().filter(|pc| constraint(*pc)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_bounds_check() {
        let range = Range {
            lower: 5,
            upper: 15,
        };

        assert!(range.in_bounds(5));
        assert!(range.in_bounds(10));
        assert!(range.in_bounds(15));

        assert!(!range.in_bounds(0));
        assert!(!range.in_bounds(20));
    }

    #[test]
    fn parse_password_constraint() {
        let input = "1-3 a: abcde";
        let constraint = PasswordConstraint::try_from(input).unwrap();

        assert_eq!(constraint.range.lower, 1);
        assert_eq!(constraint.range.upper, 3);
        assert_eq!(constraint.letter, 'a');
        assert_eq!(constraint.password, "abcde");
    }
}
