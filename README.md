# stl_io
[![Build Status](https://travis-ci.org/hmeyer/stl_io.svg?branch=master)](https://travis-ci.org/hmeyer/stl_io)
[![Codecov](https://codecov.io/github/hmeyer/stl_io/coverage.svg?branch=master)](https://codecov.io/github/hmeyer/stl_io)
[![Cargo](https://img.shields.io/crates/v/stl_io.svg)](https://crates.io/crates/stl_io)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Downloads](https://img.shields.io/crates/d/stl_io.svg)](#downloads)


stl_io is crate for reading and writing [STL (STereoLithography)](https://en.wikipedia.org/wiki/STL_(file_format)) files. It can read both, binary and ascii STL in a safe manner. Writing is limited to binary STL, which is more compact anyway.

# Examples
Read STL file:

```rust
use std::fs::OpenOptions;
let mut file = OpenOptions::new().read(true).open("mesh.stl").unwrap();
let stl = stl_io::read_stl(&mut file).unwrap();
```

Write STL file:

```rust
use std::fs::OpenOptions;
let mesh = [stl_io::Triangle { normal: [1.0, 0.0, 0.0],
                               vertices: [[0.0, -1.0, 0.0],
                                          [0.0, 1.0, 0.0],
                                          [0.0, 0.0, 0.5]]}];
let mut file = OpenOptions::new().write(true).create_new(true).open("mesh.stl").unwrap();
stl_io::write_stl(&mut file, mesh.iter()).unwrap();
```

For more information, check out the [Documentation](https://docs.rs/stl_io/).

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
