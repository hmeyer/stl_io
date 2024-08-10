# stl_io
![test workflow](https://github.com/hmeyer/stl_io/actions/workflows/test.yml/badge.svg?branch=master)
![build workflow](https://github.com/hmeyer/stl_io/actions/workflows/build.yml/badge.svg?branch=master)
[![Cargo](https://img.shields.io/crates/v/stl_io.svg)](https://crates.io/crates/stl_io)
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
Licensed under the <a href="LICENSE">MIT license</a>.
</sup>
