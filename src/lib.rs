// Copyright (c) 2017 FaultyRAM
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! A helper library for use in the Google Code Jam.

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
use std::path::{Path, PathBuf};

/// A set of test cases loaded from an input file.
#[derive(Debug)]
pub struct TestCases {
    /// An iterator over the lines of an input file.
    input: Lines<BufReader<File>>,
    /// An output file.
    output: LineWriter<File>,
    /// The current test case.
    current_case: usize,
    /// The total number of test cases to handle.
    case_count: usize,
}

/// Supports reading lines from an input file and writing data to an output file.
#[derive(Debug)]
pub struct TestCaseIo<'a>(&'a mut TestCases);

impl TestCases {
    /// Obtains a set of test cases.
    pub fn new() -> TestCases {
        let arg = env::args().nth(1).expect("input file path not specified");
        let input_file_path: &Path = arg.as_ref();
        let output_file_path: PathBuf = input_file_path.with_extension("out");
        assert_ne!(input_file_path, output_file_path);
        let mut test_cases = TestCases {
            input: Self::open_input_file(input_file_path),
            output: Self::open_output_file(&output_file_path),
            current_case: 1,
            case_count: 0,
        };
        test_cases.init_test_case_count();
        test_cases
    }

    /// Consumes a set of test cases, calling a closure once for each test case.
    pub fn run<F: Fn(&mut TestCaseIo)>(mut self, f: F) {
        let mut tc_io = TestCaseIo::new(&mut self);
        while tc_io.0.current_case <= tc_io.0.case_count {
            let current_case_s = tc_io.0.current_case_string();
            let _ = tc_io.write(current_case_s.as_bytes())
                .expect("could not write current test case string to output file");
            (f)(&mut tc_io);
            tc_io.0.current_case.add_assign(1);
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

    /// Reads a line of text from the input file.
    fn read_line(&mut self) -> String {
        self.input
            .next()
            .expect("reached end of file")
            .expect("could not read from input file")
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

impl Default for TestCases {
    fn default() -> TestCases {
        Self::new()
    }
}

impl<'a> TestCaseIo<'a> {
    /// Reads a line from the input file.
    pub fn read_line(&mut self) -> String {
        self.0.read_line()
    }

    /// Returns the current test case number.
    pub fn current_case(&self) -> usize {
        self.0.current_case
    }

    /// Returns the total number of test cases in the input file.
    pub fn case_count(&self) -> usize {
        self.0.case_count
    }

    /// Creates a new `TestCaseIo` instance.
    fn new(test_cases: &'a mut TestCases) -> TestCaseIo<'a> {
        TestCaseIo(test_cases)
    }
}

impl<'a> Write for TestCaseIo<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.output.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.output.flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.0.output.write_all(buf)
    }

    fn write_fmt(&mut self, fmt: Arguments) -> io::Result<()> {
        self.0.output.write_fmt(fmt)
    }
}
