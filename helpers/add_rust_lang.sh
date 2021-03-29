#!/usr/bin/env bash
######################################################################
#
# add_rust_lang.sh - Install Rust toolchain for the project.
#
# Copyright 2020-2021 Naman Bishnoi
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
######################################################################
set -euxo

echo "**********************************************************************"
echo "Installing Rustup v%%RUSTUP_VERSION%%"
echo "**********************************************************************"
dpkgArch="$(dpkg --print-architecture)"; 
    case "${dpkgArch##*-}" in 
        amd64) rustArch='x86_64-unknown-linux-gnu'; rustupSha256='ed7773edaf1d289656bdec2aacad12413b38ad0193fff54b2231f5140a4b07c5' ;; 
        armhf) rustArch='armv7-unknown-linux-gnueabihf'; rustupSha256='7a7b9d246ad63358705d8d4a7d5c2ef1adfec24525d1d5c44a7739e1b867e84d' ;; 
        arm64) rustArch='aarch64-unknown-linux-gnu'; rustupSha256='f80a0a792b3ab905ab4919474daf4d3f60e574fc6987e69bfba2fd877241a8de' ;; 
        i386) rustArch='i686-unknown-linux-gnu'; rustupSha256='4473c18286aa1831683a772706d9a5c98b87a61cc014d38063e00a63a480afef' ;; 
        *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; 
    esac
# Download the Rustup installer script
url="https://static.rust-lang.org/rustup/archive/1.23.1/${rustArch}/rustup-init"
curl -O "$url"

# Verify Rustup checksum for integrity
echo "${rustupSha256} *rustup-init" | sha256sum -c -

# Make Rustup executable
chmod +x rustup-init

echo "**********************************************************************"
echo "Installing Rust v1.51.0"
echo "**********************************************************************"
RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain 1.51.0 --default-host ${rustArch}
rm rustup-init

# Grants permission for particular directories
chmod -R a+w /usr/local/rustup /usr/local/cargo

echo "**********************************************************************"
echo "Installation Completed"
echo "**********************************************************************"
rustup --version
cargo --version
rustc --version
