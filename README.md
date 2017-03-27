# gcj-helper

[![Travis](https://img.shields.io/travis/FaultyRAM/gcj-helper-rs.svg)][1]
[![AppVeyor](https://img.shields.io/appveyor/ci/FaultyRAM/gcj-helper-rs.svg)][2]
[![Crates.io](https://img.shields.io/crates/v/gcj-helper.svg)][3]
[![Docs.rs](https://docs.rs/gcj-helper/badge.svg)][4]

`gcj-helper` is a [Rust][5] library for writing [Google Code Jam][6] solutions. It handles the
boilerplate so you can focus on writing solutions instead.

## Examples

### Sequential computation

```rust
extern crate gcj_helper;

use gcj_helper::TestEngine;
use std::io::Write;

fn main() {
    TestEngine::new("./foo.in", "./foo.out").run(|input, output| {
        writeln!(output, " {}", input.read_next_line())
    });
}
```

### Parallel computation

```rust
extern crate gcj_helper;

use gcj_helper::TestEngine;

fn main() {
    TestEngine::new("./foo.in", "./foo.out")
        .run_parallel(|input| input.read_next_line(), |data| format!(" {}\n", data));
}
```

## Usage

### Via `cargo new --template`

For brand-new crates, the quickest way to get started is to use a Cargo template:

```text
cargo new --template https://github.com/FaultyRAM/gcj-template-rust.git foobar
```

This creates a new crate named `foobar` that uses `gcj-helper`'s sequential API. If you want to use
the parallel API instead, in `Cargo.toml` replace this line:

```toml
gcj-helper = "0.3"
```

With this line:

```toml
gcj-helper = { version = "0.3", features = ["parallel"] }
```

And in `src/main.rs`, replace the call to `TestEngine::new()` with a call to
`TestEngine::new_parallel()` as shown above.

### By hand

You can also manually add `gcj-helper` to your crate, though doing so is slower than using
`cargo new --template`. To do this, either add this to your `[dependencies]` in `Cargo.toml`:

```toml
gcj-helper = "0.3"
```

Or if you want to use the parallel API, add this instead:

```toml
gcj-helper = { version = "0.3", features = ["parallel"] }
```

After adding `gcj-helper` to `Cargo.toml`, add this to your crate root:

```rust
extern crate gcj_helper;
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[1]: https://travis-ci.org/FaultyRAM/gcj-helper-rs
[2]: https://ci.appveyor.com/project/FaultyRAM/gcj-helper-rs
[3]: https://crates.io/crates/gcj-helper
[4]: https://docs.rs/gcj-helper
[5]: https://www.rust-lang.org
[6]: https://code.google.com/codejam/
