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

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", forbid(clippy))]
#![cfg_attr(feature = "clippy", forbid(clippy_internal))]
#![cfg_attr(feature = "clippy", forbid(clippy_pedantic))]
#![cfg_attr(feature = "clippy", forbid(clippy_restrictions))]
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

use std::{env, io};
use std::fmt::Arguments;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, LineWriter, Lines, Write};
use std::ops::{Add, AddAssign};
use std::path::Path;

/// Facilitates the execution of problem solving code.
///
/// In order to handle test cases, you need to create a new `TestEngine`, then call
/// `TestEngine::run()`, which accepts an `Fn(&mut IoHelper)` that is called once per test case.
/// Creating a `TestEngine` is cheap; no files are opened until you call `TestEngine::run()`.
#[derive(Debug)]
pub struct TestEngine<I: AsRef<Path>, O: AsRef<Path>> {
    /// A path to an input file.
    input_file_path: I,
    /// A path to an output file.
    output_file_path: O,
}

/// Provides I/O support for problem solving code.
///
/// The `IoHelper` type allows reading from an input file and writing to an output file.
/// `IoHelper::read_line()` reads the next line of text from the input file, and `io::Write` is
/// implemented for `IoHelper` such that data is written to the output file.
#[derive(Debug)]
pub struct IoHelper {
    /// An iterator over the lines of an input file.
    input: Lines<BufReader<File>>,
    /// An output file.
    output: LineWriter<File>,
    /// The current test case.
    current_case: usize,
    /// The total number of test cases to handle.
    case_count: usize,
}

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

    /// Consumes the test engine, calling a closure once for each test case.
    pub fn run<F: Fn(&mut IoHelper)>(self, f: F) {
        let io_helper = IoHelper::new(self);
        io_helper.run(f);
    }
}

impl TestEngine<String, String> {
    /// Creates a new test engine using input and output file paths obtained from command line
    /// arguments.
    ///
    /// Calling this method is cheap; no files are opened until `TestEngine::run()` is called.
    ///
    /// # Panics
    ///
    /// This method panics if either the input file path or output file path is missing.
    pub fn from_args() -> TestEngine<String, String> {
        let mut args = env::args();
        let input_file_path = args.nth(1).expect("input file path not specified");
        let output_file_path = args.next().expect("output file path not specified");
        Self::new(input_file_path, output_file_path)
    }
}

impl Default for TestEngine<String, String> {
    fn default() -> TestEngine<String, String> {
        Self::from_args()
    }
}

impl IoHelper {
    /// Reads a line of text from the input file.
    ///
    /// # Panics
    ///
    /// This method panics if reading fails for any reason, such as having reached the end of the
    /// input file.
    pub fn read_line(&mut self) -> String {
        self.input
            .next()
            .expect("reached end of file")
            .expect("could not read from input file")
    }

    /// Returns the current test case number.
    pub fn current_case(&self) -> usize {
        self.current_case
    }

    /// Returns the total number of test cases in the input file.
    pub fn case_count(&self) -> usize {
        self.case_count
    }

    /// Creates a new I/O helper.
    fn new<I: AsRef<Path>, O: AsRef<Path>>(test_engine: TestEngine<I, O>) -> IoHelper {
        let mut io_helper = IoHelper {
            input: Self::open_input_file(test_engine.input_file_path),
            output: Self::open_output_file(test_engine.output_file_path),
            current_case: 1,
            case_count: 0,
        };
        io_helper.init_test_case_count();
        io_helper
    }

    /// Consumes the I/O helper, calling a closure once for each test case.
    fn run<F: Fn(&mut IoHelper)>(mut self, f: F) {
        while self.current_case <= self.case_count {
            let current_case_s = self.current_case_string();
            let _ = self.write(current_case_s.as_bytes())
                .expect("could not write current test case string to output file");
            (f)(&mut self);
            self.current_case.add_assign(1);
        }
    }

    /// Creates an iterator over the lines of an input file.
    fn open_input_file<P: AsRef<Path>>(path: P) -> Lines<BufReader<File>> {
        BufReader::new(OpenOptions::new()
            .read(true)
            .open(path)
            .expect("could not open input file for reading")).lines()
    }

    /// Creates an output file.
    fn open_output_file<P: AsRef<Path>>(path: P) -> LineWriter<File> {
        LineWriter::new(OpenOptions::new()
                            .write(true)
                            .truncate(true)
                            .create(true)
                            .open(path)
                            .expect("could not open output file for writing"))
    }

    /// Obtains the number of test cases to examine from the input file.
    fn init_test_case_count(&mut self) {
        let line = self.read_line();
        self.case_count =
            usize::from_str_radix(&line, 10).expect("could not parse test case count");
    }

    /// Creates a string identifying the current test case.
    fn current_case_string(&mut self) -> String {
        let case_str = "Case #";
        let case_s = self.current_case.to_string();
        let case_str_colon = ":";
        let mut out = String::with_capacity(0);
        out.reserve_exact(case_str.len().add(case_s.len()).add(case_str_colon.len()));
        out.push_str(case_str);
        out.push_str(&case_s);
        out.push_str(case_str_colon);
        out
    }
}

impl Write for IoHelper {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.output.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.output.write_all(buf)
    }

    fn write_fmt(&mut self, fmt: Arguments) -> io::Result<()> {
        self.output.write_fmt(fmt)
    }
}
