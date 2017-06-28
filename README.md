# stl_io
[![Build Status](https://travis-ci.org/hmeyer/stl_io.svg?branch=master)](https://travis-ci.org/hmeyer/stl_io)

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
