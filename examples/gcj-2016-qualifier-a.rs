// Copyright (c) 2017 FaultyRAM
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! An example Google Code Jam solution using `gcj-helper`.
//!
//! This is a naive solution for *Counting Sheep*, a.k.a. Problem A from the Qualification Round
//! of Google Code Jam 2016.

extern crate gcj_helper;

use gcj_helper::{TestCaseIo, TestCases};
use std::io::Write;

fn insomnia(tc_io: &mut TestCaseIo) {
    writeln!(tc_io, " INSOMNIA").expect("could not write to output file");
}

fn result(tc_io: &mut TestCaseIo, number: &str) {
    writeln!(tc_io, " {}", number).expect("could not write to output file");
}

fn main() {
    let tc = TestCases::new();
    tc.run(|tc_io| {
        let mut digits_found = [false; 10];
        let mut digits_count = 0;
        let mut step = tc_io.read_line();
        let mut step_mul = 2;
        let number = u32::from_str_radix(&step, 10).expect("could not parse test case");
        if step == "0" {
            insomnia(tc_io);
            return;
        }
        loop {
            for digit in step.bytes() {
                match digit {
                    0x30...0x39 => {
                        let i = (digit - 0x30) as usize;
                        if !digits_found[i] {
                            digits_found[i] = true;
                            digits_count += 1;
                            if digits_count >= digits_found.len() {
                                result(tc_io, &step);
                                return;
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
            step = (number * step_mul).to_string();
            step_mul += 1;
        }
    });
}
