<!-- Project Metadata -->
<!-- project_tags: api, opencl, gpu, gpgpu, rust -->
<!-- project_featured: true -->

# ⚙️ opencl-rs

[![N3NX](https://img.shields.io/badge/n3n-org-blueviolet.svg)](https://n3n.org)
[![N3NX](https://img.shields.io/badge/discord-n3n-%237289da.svg?logo=discord)](https://discord.gg/QNDkGDTs)
[![Crates.io](https://img.shields.io/crates/v/opencl-heads.svg)](https://crates.io/crates/opencl-heads)
[![Docs](https://docs.rs/opencl-heads/badge.svg)](https://docs.rs/opencl-heads)
[![dependency status](https://deps.rs/repo/github/n3nx/opencl-rs/status.svg)](https://deps.rs/repo/github/n3nx/opencl-rs)
[![Build status](https://github.com/n3nx/opencl-rs/workflows/CI/badge.svg)](https://github.com/n3nx/opencl-rs/actions)

## OpenCL Collections Library

Library that contains OpenCL APIs, Adapters, FFI headers and related utilities.

This library is intended to support future version of OpenCL while maintaining long term support for Rust.

By **Long Term Support**, we commit to fix bugs and to _actively maintain_ proper functionality of this library over the years. We also plan to create several milestones for this project to have _sustainable development_ over long period of time.

## Features

This library is intended to provide these features alongside of traditional ones.

1. Compatible with Latest OpenCL Versions (**OpenCL 3.0**)
2. Supports Compatiblity with Older and Deprecated APIs for Previous Versions (**OpenCL 1.1, 1.2 etc.**)
3. Lightweight Headers and API Objects
4. Superior Error Handling
5. Periodic Bug Fix Management
6. Optimised for High Performance
7. Ease of Use and Simplicity
8. Types and Functions Safety

## Installation

This library can be installed in two ways.

1. Precompiled library files (for major platforms), you can download those from the [release section](https://github.com/diabloxenon/opencl/releases/) of this repository.
2. Build it yourself (needs cargo installed obviously), instructions are simple: `cargo build --release` and to test the functions on your platform: `cargo test`

## Upgrading

The N3N follows Semantic Versioning for their projects. There are 3 types of release lifecycle associated with this project, described as below:

1. **Major** releases `x.0.0` supports the version jump from Khronos OpenCL. Before updating to these versions, be sure to review changelog first.
2. **Minor** release `0.x.0` integrates new library features or Khronos OpenCL Header updates. Generally these changes are safe and are non-disruptive to previous releases.
3. **LTS** releases `0.0.x` supports critical bug fix and patches and are safe to update to these versions without second thought.

Please refer to [VENDORING](/VENDORING.md) document for more details.

## Contribution

[![N3N Contributor Guidelines](https://img.shields.io/badge/N3N%20Guidelines-v1.0-ff69b4.svg)](./CODE_OF_CONDUCT.md)

We welcome community contributions to this project.

Please read our [Contributor Guide](CONTRIBUTING.md) for more information on how to get started.
Please also read our [Contributor Terms](CONTRIBUTING.md#contributor-terms) before you make any contributions.

Any contribution intentionally submitted for inclusion in a N3N project or a Knytx Labs project, shall comply with the Rust standard licensing model (MIT OR Apache 2.0) and therefore be dual licensed as described below, without any additional terms or conditions:

### License

This contribution is dual licensed under EITHER OF

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

For clarity, "your" refers to the N3N contributors, NCRYPTO Labs Private Limited or any other licensee/user of the contribution.

## Credits

See `CREDITS.md` file for heartiest thanks and acknowledgement to other awesome developers out there.

The OpenCL and the OpenCL logo are trademarks of Apple Inc. used by permission by Khronos. Complete license details are available on the [Khronos](https://www.khronos.org/legal/trademarks/) and [Apple](https://developer.apple.com/softwarelicensing/opencl/) website.

This project is initiated by [Naman Bishnoi](https://twitter.com/namanbishnoi) [@diabloxenon](https://twitter.com/diabloxenon)
