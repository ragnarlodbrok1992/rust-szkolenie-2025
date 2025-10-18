// Create a program that
// 1. Takes a user console input for integer (whole number larger than 0)
// 2. Finds it's prime factors (prime numbers that divide it)
// 3. Creates a FizzBuzz output that prints out Fizz for smallest prime factor and Buzz for largest
//    prime factor. FizzBuzz for both.
// 4. Prints out elements.

use std::char::ParseCharError;
use std::fmt::Display;
use std::io;
use std::io::{Error};
use std::num::{ParseFloatError, ParseIntError};
use std::str::ParseBoolError;

fn main() {
    let factorizer = TrialDivisionPrimeFactorizer::new();

    let app = CliApp::new(
        "FizzBuzz with Prime Factorization".to_string(),
        "1.0.0".to_string(),
        factorizer
    );

    app.run();
}

pub struct CliApp<T: PrimeFactorizer> {
    app_name: String,
    app_version: String,
    factorizer: T,
}

impl<T: PrimeFactorizer> CliApp<T> {
    pub fn new(app_name: String, app_version: String, factorizer: T) -> Self {
        Self {
            app_name,
            app_version,
            factorizer,
        }
    }

    pub fn run(&self) {
        println!("{} v{}", self.app_name, self.app_version);
        match self.get_user_input_from_stdin() {
            Ok(input_number) => {
                println!("You entered: {}", input_number);
                let result = self.factorizer.factorize(input_number);

                match result {
                    None => println!("No prime factors found."),
                    Some(result) => {
                        println!("Result debug: {:?}", result);

                        println!("{}", result);
                    }
                }
            }
            Err(e) => match e {
                InputError::InvalidFormat(msg) => eprintln!("Invalid format: {}", msg),
                InputError::OutOfRange(msg) => eprintln!("Out of range: {}", msg),
                InputError::IoError(err) => eprintln!("I/O error: {}", err),
            },
        }
    }

    fn get_user_input_from_stdin(&self) -> Result<InputNumber, InputError> {
        println!("Please enter a whole number larger than 0:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let value: u32 = input.trim().parse()?;

        InputNumber::new(value)
    }
}

pub trait PrimeFactorizer {
    fn factorize(&self, number: InputNumber) -> Option<FactorizationResult>;
}

pub struct TrialDivisionPrimeFactorizer {}

impl TrialDivisionPrimeFactorizer {
    pub fn new() -> Self {
        Self {}
    }
}

impl PrimeFactorizer for TrialDivisionPrimeFactorizer {
    fn factorize(&self, number: InputNumber) -> Option<FactorizationResult> {
        let mut n: u32 = number.into();
        if n < 2 {
            return None;
        }

        let mut factors = Vec::new();
        let mut divisor = 2;

        while n >= 2 {
            if n % divisor == 0 {
                factors.push(divisor);
                n /= divisor;
            } else {
                divisor += 1;
            }
        }

        FactorizationResult::new(factors)
    }
}

#[derive(Debug)]
pub struct FactorizationResult {
    pub factors: Vec<u32>,
    pub smallest: u32,
    pub largest: u32,
}

impl FactorizationResult {
    pub fn new(factors: Vec<u32>) -> Option<Self> {
        if factors.is_empty() {
            return None;
        }

        let smallest = *factors.iter().min().expect("Factors should not be empty");
        let largest = *factors.iter().max().expect("Factors should not be empty");

        Some(Self {
            factors,
            smallest,
            largest,
        })
    }
}

impl Display for FactorizationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Prime factors with their FizzBuzz roles:")?;

        if self.smallest == self.largest {
            for factor in &self.factors {
                writeln!(f, "- {} (FizzBuzz)", factor)?;
            }
        } else {
            for factor in &self.factors {
                match *factor {
                    n if n == self.smallest => writeln!(f, "- {} (Fizz)", n)?,
                    n if n == self.largest => writeln!(f, "- {} (Buzz)", n)?,
                    _ => writeln!(f, "- {}", factor)?,
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct InputNumber(u32);

impl InputNumber {
    pub fn new(value: u32) -> Result<Self, InputError> {
        if value == 0 {
            return Err(InputError::OutOfRange("Number must be larger than 0".to_string()));
        }

        Ok(InputNumber(value))
    }
}

impl Display for InputNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<InputNumber> for u32 {
    fn from(input: InputNumber) -> Self {
        input.0
    }
}

pub enum InputError {
    InvalidFormat(String),
    OutOfRange(String),
    IoError(Error),
}

impl From<Error> for InputError {
    fn from(error: Error) -> Self {
        InputError::IoError(error)
    }
}

impl From<ParseBoolError> for InputError {
    fn from(error: ParseBoolError) -> Self {
        InputError::InvalidFormat(error.to_string())
    }
}

impl From<ParseIntError> for InputError {
    fn from(error: ParseIntError) -> Self {
        InputError::InvalidFormat(error.to_string())
    }
}

impl From<ParseCharError> for InputError {
    fn from(error: ParseCharError) -> Self {
        InputError::InvalidFormat(error.to_string())
    }
}

impl From<ParseFloatError> for InputError {
    fn from(error: ParseFloatError) -> Self {
        InputError::InvalidFormat(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_number_creation() {
        let valid_input = InputNumber::new(5);
        assert!(valid_input.is_ok());

        let invalid_input = InputNumber::new(0);
        assert!(invalid_input.is_err());
    }

    #[test]
    fn test_cli_app_creation() {
        let factorizer = TrialDivisionPrimeFactorizer::new();
        let app = CliApp::new("TestApp".to_string(), "0.1.0".to_string(), factorizer);
        assert_eq!(app.app_name, "TestApp");
        assert_eq!(app.app_version, "0.1.0");
    }

    #[test]
    fn test_prime_factorization() {
        let prime_factorizer = TrialDivisionPrimeFactorizer::new();
        let cases = vec![InputNumber(28), InputNumber(15), InputNumber(13), InputNumber(100)];

        for case in cases {
            let factors = prime_factorizer.factorize(case).expect("Should have factors").factors;
            let result = factors.iter().fold(1, |acc, &x| acc * x);
            assert_eq!(result, case.into());
        }
    }

    #[test]
    fn test_factorization_result_display_different_factors() {
        let result = FactorizationResult::new(vec![2, 2, 5]).unwrap();
        let display_string = format!("{}", result);
        let expected = "Prime factors with their FizzBuzz roles:\n- 2 (Fizz)\n- 2 (Fizz)\n- 5 (Buzz)\n";
        assert_eq!(display_string, expected);
    }

    #[test]
    fn test_factorization_result_display_same_factors() {
        let result = FactorizationResult::new(vec![7]).unwrap();
        let display_string = format!("{}", result);
        let expected = "Prime factors with their FizzBuzz roles:\n- 7 (FizzBuzz)\n";
        assert_eq!(display_string, expected);
    }
}
