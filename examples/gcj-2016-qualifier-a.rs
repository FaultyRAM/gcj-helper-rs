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

use gcj_helper::TestEngine;
use std::io::Write;

fn main() {
    TestEngine::from_args().run(|input, output| {
        let mut digits_found = [false; 10];
        let mut digits_count = 0;
        let mut step = input.read_next_line();
        let mut step_mul = 2;
        let number = u32::from_str_radix(&step, 10).unwrap();
        if step == "0" {
            writeln!(output, " INSOMNIA").unwrap();
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
                                writeln!(output, " {}", step).unwrap();
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
