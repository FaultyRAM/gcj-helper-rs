// Copyright (c) 2017 FaultyRAM
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! A helper library for Google Code Jam solutions.
//!
//! In the Google Code Jam, solving a problem typically requires the following steps:
//!
//! 1. Open an input file containing a series of test cases.
//! 2. Open an output file where solutions will be written.
//! 2. Read the first line from the input file, which consists solely of an unsigned integer
//!    specifying the number of test cases in the input file.
//! 3. For each test case, perform the following steps:
//!    1. Obtain the corresponding test data by reading one or more lines from the input file (it
//!       may be a fixed number, or specified within the test data itself).
//!    2. Perform some logic using the test data, in order to obtain a set of results.
//!    3. Write the string `"Case #N:"` (where `N` is the number of completed test cases) followed
//!       by the results obtained in the previous step, formatted as the problem requires.
//!
//! Writing code to handle all of the above is tedious and time-consuming, in a situation where
//! every microsecond counts. `gcj-helper` is designed to handle the boilerplate, so you can focus
//! on writing solutions instead.
//!
//! # The `TestEngine` type
//!
//! To execute test cases, you need to create a `TestEngine` and call `TestEngine::run()`.
//! `TestEngine::run()` accepts two closures:
//!
//! 1. A `parser` that reads from an input file and returns the data for one test case.
//! 2. A `solver` that performs logic on the data for one test case and returns a result, encoded
//!    as a `Display` type.
//!
//! This two-step process to writing solutions is useful for two reasons:
//!
//! * It seperates parsing from the solution itself, making your code easier to read;
//! * It enables test case parallelisation if the `parallel` feature is enabled, improving
//!   run-time performance at the cost of increased build times.
//!
//! # The `InputReader` type
//!
//! `gcj-helper` provides parsers with access to an `InputReader`, a simple wrapper around a
//! `std::fs::File`. `InputReader` implements `io::BufRead` and also provides a convenience
//! method, `read_next_line()`, which reads a line of text from an input file and truncates the
//! end-of-line marker if one is present.
//!
//! # Formatting test results
//!
//! Before each test case, the `TestEngine` writes the string `"Case #N:"`, where `N` is the
//! current test case. This does not prepend or append any whitespace. This means that if the
//! colon must be followed by a space, your result should begin with one, and that the result must
//! end with a newline.

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", forbid(clippy))]
#![cfg_attr(feature = "clippy", forbid(clippy_internal))]
#![cfg_attr(feature = "clippy", forbid(clippy_pedantic))]
#![forbid(warnings)]
#![forbid(box_pointers)]
#![forbid(fat_ptr_transmutes)]
#![forbid(missing_copy_implementations)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_docs)]
#![forbid(trivial_casts)]
#![forbid(trivial_numeric_casts)]
#![forbid(unsafe_code)]
#![forbid(unused_extern_crates)]
#![forbid(unused_import_braces)]
#![deny(unused_qualifications)]
#![forbid(unused_results)]
#![forbid(variant_size_differences)]

#[cfg(feature = "parallel")]
extern crate rayon;

#[cfg(feature = "parallel")]
use rayon::prelude::*;
use std::{env, io};
use std::ffi::OsString;
use std::fmt::{Arguments, Display};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, LineWriter, Read, Write};
use std::path::Path;

/// Facilitates the execution of problem solving code.
#[derive(Debug)]
pub struct TestEngine<I: AsRef<Path>, O: AsRef<Path>> {
    /// A path to an input file.
    input_file_path: I,
    /// A path to an output file.
    output_file_path: O,
}

/// Supports reading from an input file.
#[derive(Debug)]
pub struct InputReader(BufReader<File>);

/// Supports writing to an output file.
struct OutputWriter(LineWriter<File>);

impl<I: AsRef<Path>, O: AsRef<Path>> TestEngine<I, O> {
    /// Creates a new test engine using the specified input and output file paths.
    ///
    /// Calling this method is cheap; no files are opened until `TestEngine::run()` is called.
    pub fn new(input_file_path: I, output_file_path: O) -> TestEngine<I, O> {
        TestEngine {
            input_file_path: input_file_path,
            output_file_path: output_file_path,
        }
    }

    #[cfg(not(feature = "parallel"))]
    /// Consumes the test engine, executing a parser and solver once per test case.
    ///
    /// # Panics
    ///
    /// This method panics in the event of an I/O error.
    pub fn run<
        D: Sized + Send + Sync,
        R: Display + Sized + Send,
        P: Fn(&mut InputReader) -> D,
        S: Fn(&D) -> R + Sync
    >
        (
        self,
        p: P,
        s: S,
    ) {
        let mut reader = InputReader::new(self.input_file_path);
        let mut writer = OutputWriter::new(self.output_file_path);
        let mut current_case: usize = 1;
        let case_count = reader.get_case_count();
        while current_case <= case_count {
            writer.write_test_result(current_case, (s)(&(p)(&mut reader)));
            current_case += 1;
        }
    }

    /// Consumes the test engine, executing a parser and solver once per test case.
    ///
    /// # Panics
    ///
    /// This method panics in the event of an I/O error.
    #[cfg(feature = "parallel")]
    pub fn run<
        D: Sized + Send + Sync,
        R: Display + Sized + Send,
        P: Fn(&mut InputReader) -> D,
        S: Fn(&D) -> R + Sync
    >
        (
        self,
        p: P,
        s: S,
    ) {
        let mut reader = InputReader::new(self.input_file_path);
        let mut writer = OutputWriter::new(self.output_file_path);
        let case_count = reader.get_case_count();
        let mut data = Vec::with_capacity(0);
        data.reserve_exact(case_count);
        for _ in 0..case_count {
            data.push((p(&mut reader), None));
        }
        data.par_iter_mut().for_each(|d| d.1 = Some(s(&d.0)));
        for (i, &(_, ref r)) in data.iter().enumerate() {
            writer.write_test_result(
                i + 1,
                match *r {
                    Some(ref x) => x,
                    None => unreachable!(),
                },
            );
        }
    }
}

impl TestEngine<OsString, OsString> {
    /// Creates a new test engine using input and output file paths obtained from command line
    /// arguments.
    ///
    /// Calling this method is cheap; no files are opened until `TestEngine::run()` is called.
    ///
    /// # Panics
    ///
    /// This method panics if either the input file path or output file path is missing.
    pub fn from_args() -> TestEngine<OsString, OsString> {
        let mut args = env::args_os();
        let input_file_path = args.nth(1).expect("input file path not specified");
        let output_file_path = args.next().expect("output file path not specified");
        Self::new(input_file_path, output_file_path)
    }
}

impl Default for TestEngine<OsString, OsString> {
    fn default() -> TestEngine<OsString, OsString> {
        Self::from_args()
    }
}

impl InputReader {
    /// Reads a line of text from the input file, consuming the end-of-line marker if one is
    /// present.
    pub fn read_next_line(&mut self) -> String {
        let mut line = String::with_capacity(0);
        let _ = self.read_line(&mut line)
            .expect("could not read from input file");
        let line_len = line.lines()
            .next()
            .map(|l| l.len())
            .expect("could not obtain line length");
        line.truncate(line_len);
        line
    }

    /// Creates a new input reader over the given input file.
    fn new<P: AsRef<Path>>(path: P) -> InputReader {
        InputReader(
            BufReader::new(
                OpenOptions::new()
                    .read(true)
                    .open(path)
                    .expect("could not open input file for reading"),
            ),
        )
    }

    /// Reads the number of test cases from the input file.
    fn get_case_count(&mut self) -> usize {
        usize::from_str_radix(&self.read_next_line(), 10).expect("could not parse test case count")
    }
}

impl Read for InputReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.0.read_to_end(buf)
    }

    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        self.0.read_to_string(buf)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.0.read_exact(buf)
    }
}

impl BufRead for InputReader {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.0.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.0.consume(amt)
    }

    fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.0.read_until(byte, buf)
    }

    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        self.0.read_line(buf)
    }
}

impl OutputWriter {
    /// Creates a new output writer over the given output file.
    fn new<P: AsRef<Path>>(path: P) -> OutputWriter {
        OutputWriter(
            LineWriter::new(
                OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .create(true)
                    .open(path)
                    .expect("could not open output file for writing"),
            ),
        )
    }

    /// Writes a test result to the output file.
    fn write_test_result<R: Display>(&mut self, case: usize, result: R) {
        let case_prefix = "Case #";
        let case_number = case.to_string();
        let case_colon = ":";
        let r = result.to_string();
        let mut output = String::with_capacity(0);
        output.reserve_exact(case_prefix.len() + case_number.len() + case_colon.len() + r.len(),);
        output.push_str(case_prefix);
        output.push_str(&case_number);
        output.push_str(case_colon);
        output.push_str(&r);
        self.write_all(output.as_bytes())
            .expect("could not write test result to output file");
    }
}

impl Write for OutputWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.0.write_all(buf)
    }

    fn write_fmt(&mut self, fmt: Arguments) -> io::Result<()> {
        self.0.write_fmt(fmt)
    }
}
