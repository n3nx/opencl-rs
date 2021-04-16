# OpenCL Heads

**NOTE:** This project is currently under heavy development, we advise you to use the version 0.3.x onwards for your project dependencies. Thank you for your support!

## Description

Library that contains OpenCL FFI headers.

This library is intended to support future version of OpenCL while maintaining long term support for Rust.

By **Long Term Support**, we commit to fix bugs and to _actively maintain_ proper functionality of this library over the years. We also plan to create several milestones for this project to have _sustainable development_ over long period of time.

## Features

This library is intended to provide these features alongside of traditional ones.

1. Compatible with latest OpenCL version (**OpenCL 3.0**)
2. Lightweight Headers
3. Periodic Bug Fix Management
4. Optimizations for High Performance
5. Ease of Use and Simplicity
6. Types and Functions Safety

## Installation

This library can be installed in two ways.

1. Precompiled library files (for major platforms), you can download those from the [release section](https://github.com/diabloxenon/opencl/releases/) of this repository.
2. Build it yourself (needs cargo installed obviously), instructions are simple: `cargo build --release` and to test the functions on your platform: `cargo test`

## Updating

There are 3 types of release lifecycle associated with this project, described as below:

1. **Major** releases `x.0.0` supports the version jump from Khronos OpenCL. Before updating to these versions, be sure to review changelog first.
2. **Minor** release `0.x.0` integrates new library features or Khronos OpenCL Header updates. Generally these changes are safe and are non-disruptive to previous releases.
3. **LTS** releases `0.0.x` supports critical bug fix and patches and are safe to update to these versions without second thought.

## License

Project is Apache-2.0 licensed, please read `LICENSE` file to know more about the license.

```rust
/*
 * Copyright 2020-2021 Naman Bishnoi
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/
```

## Credits

See `CREDITS.md` file for heartiest thanks and acknowledgement to other awesome developers out there.

The OpenCL and the OpenCL logo are the registered trademark of Apple Inc.

This project is made with 🩸 💦 😢 by [Naman Bishnoi](https://twitter.com/namanbishnoi) [@diabloxenon](https://twitter.com/diabloxenon)
