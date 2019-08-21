//! The module for describing recoverable errors in `csvpivot`.
//!
//! > *Note:* All of the error handling for `csvpivot` is structured from 
//! > [this error handling guide](https://blog.burntsushi.net/rust-error-handling)
//! > and from the source code of the [csv crate](https://github.com/BurntSushi/rust-csv)
//! > in Rust. If you're hoping to implement you're own library or binary in Rust,
//! > I highly recommend both (and, especiialy, the guide).
//!
//! You can characterize all four error types in two general categories: 
//! errors configuring the CSV reader and errors parsing individual lines.
//! For errors relating to configuration, my goal is simply to be as specific
//! and clear as possible about the nature of a given error. For errors relating to
//! parsing, however, I also think it's important to display record numbers to help
//! users debug errors they run into. Currently, this refers to the 1-indexed number in
//! which a record appears in a CSV document. So record 5 of a CSV would be the sixth line
//! of a CSV with a header row (again, 1-indexed) and the fifth line of a CSV without a header row.
//!
//! If you plan on altering the error handling in `csvpivot`, whether because you think
//! a particular error message is confusing or because the current program panics under some condition(s),
//! I want you to stick to this approach. 

extern crate csv;

use std::error::Error;
use std::fmt;
use std::io;
use std::num;

#[derive(Debug)]
pub enum CsvPivotError {
    /// Errors from reading a CSV file. 
    ///
    /// This should be limited to inconsistencies in the number of lines appearing in a given row.
    CsvError(csv::Error),
    /// Errors in the initial configuration from command-line arguments.
    ///
    /// This error likely occurs most frequently because of problems in how fields are named
    /// but can also occur because of errors parsing delimiters as single UTF-8 characters.

    InvalidConfiguration(String),
    /// A standard IO error. Typically from trying to read a file that does not exist
    Io(io::Error),
    /// Errors trying to parse a new value. 

    /// The way in which `csvpivot` parses values depends on the aggregation function
    /// and command-line flags, but all errors in converting the string records in the values
    /// column into a particular data type result in a `ParsingError`.
    ParsingError {
        line_num: usize,
        err: String,
    }
}

impl fmt::Display for CsvPivotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CsvPivotError::CsvError(ref err) => err.fmt(f),
            CsvPivotError::InvalidConfiguration(ref err) => {
                write!(f, "Could not properly configure the aggregator: {}", err)
            }
            CsvPivotError::Io(ref err) => err.fmt(f),
            // adapted from https://github.com/BurntSushi/rust-csv/blob/master/src/error.rs
            CsvPivotError::ParsingError { line_num: ref line_num, err: ref err } => {
                write!(
                    f,
                    "Could not parse record {}: {}",
                    line_num + 1,
                    err
                )
            },
        }
    }
}

impl Error for CsvPivotError {
    fn description(&self) -> &str {
        match *self {
            CsvPivotError::CsvError(ref err) => err.description(),
            CsvPivotError::Io(ref err) => err.description(),
            CsvPivotError::InvalidConfiguration(ref _err) => "could not configure the aggregator",
            CsvPivotError::ParsingError {line_num: ref _num, err: ref _err } => "failed to parse values column",
        }
    }
}

impl From<io::Error> for CsvPivotError {
    fn from(err: io::Error) -> CsvPivotError {
        CsvPivotError::Io(err)
    }
}

impl From<csv::Error> for CsvPivotError {
    fn from(err: csv::Error) -> CsvPivotError {
        CsvPivotError::CsvError(err)
    }
}