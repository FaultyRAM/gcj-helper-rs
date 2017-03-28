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
use std::fmt::Arguments;
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
#[derive(Debug)]
pub struct OutputWriter(LineWriter<File>);

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

    /// Consumes the test engine, executing tests cases in sequence.
    ///
    /// # Panics
    ///
    /// This method panics in the event of an I/O error.
    pub fn run<F: Fn(&mut InputReader, &mut OutputWriter)>(self, f: F) {
        let mut reader = InputReader::new(self.input_file_path);
        let mut writer = OutputWriter::new(self.output_file_path);
        let mut current_case: usize = 1;
        let case_count = reader.get_case_count();
        while current_case <= case_count {
            writer.write_case_number(current_case);
            (f)(&mut reader, &mut writer);
            current_case += 1;
        }
    }

    /// Consumes the test engine, executing test cases in parallel.
    ///
    /// # Panics
    ///
    /// This method panics in the event of an I/O error.
    #[cfg(feature = "parallel")]
    pub fn run_parallel<
        D: Sized + Send + Sync,
        P: Fn(&mut InputReader) -> D,
        S: Fn(&D) -> String + Sync
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
            data.push((p(&mut reader), String::with_capacity(0)));
        }
        data.par_iter_mut().for_each(|d| d.1 = s(&d.0));
        for (i, d) in data.iter().enumerate() {
            write!(writer, "Case #{}:{}", i + 1, d.1)
                .expect("could not write test result to output file");
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

    /// Writes the string "Case #N:" (where `N` is the given case number) to the output file.
    fn write_case_number(&mut self, case: usize) {
        self.write_all(b"Case #")
            .expect("could not write case number to output file");
        self.write_all(case.to_string().as_bytes())
            .expect("could not write case number to output file");
        self.write_all(b":")
            .expect("could not write case number to output file");
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
