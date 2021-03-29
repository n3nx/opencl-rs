#!/usr/bin/python
######################################################################
#
# helpers.py - Assorted utilites and helper functions for maintainance.
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

import re
import os
from urllib import request
from collections import namedtuple

## Constants ##
repo_name = "https://github.com/diabloxenon/opencl"
rust_version = "1.51.0"
rustup_version = "1.23.1"
rustup_home = "/usr/local/rustup"
cargo_home = "/usr/local/cargo"
path = "/usr/local/cargo/bin:$PATH"
pwd = os.getcwd()

DebianArch = namedtuple("DebianArch", ["bashbrew", "dpkg", "rust", "docker"])

debian_arches = [
    DebianArch("amd64", "amd64", "x86_64-unknown-linux-gnu", "ubuntu:20.04"),
    DebianArch("arm32v7", "armhf", "armv7-unknown-linux-gnueabihf",
               "arm32v7/ubuntu:20.04"),
    DebianArch("arm64v8", "arm64", "aarch64-unknown-linux-gnu",
               "arm64v8/ubuntu:20.04"),
    DebianArch("i386", "i386", "i686-unknown-linux-gnu", "i386/ubuntu:18.04"),
]


def update_versions():
    project_version = read_version()
    update_cargo_version(project_version)
    build_version_update(project_version)
    drone_build_update(project_version)


def update_all():
    dockerfile_image_generate()
    generate_rust_helper()


def update_cargo_version(version):
    ver = 'version = "' + version + '"'
    for dirs in os.walk(pwd):
        dir_name, _, files = dirs
        if "Cargo.toml" in files:
            location = dir_name + "/" + "Cargo.toml"
            config_data = read_file(location)
            config_upd = re.sub(
                r'\nversion[\s|=|"]+\w.\w.\w"\n', "\n"+ver+"\n", config_data)
            config_upd = re.sub(
                r'[{|\s|=]+version[\s|=|"]+\w.\w.\w",', " = { "+ver+",", config_upd)
            write_file(location, config_upd)


def drone_build_update(version):
    template = read_file(pwd+'/helpers/templates/drone.temp')
    rendered = template
    for arch in debian_arches:
        image = generate_image_name(version, arch.dpkg)
        rendered = rendered.replace(
            "%%IMAGE-VERSION-"+arch.dpkg.upper()+"%%", image)
    write_file(pwd+'/.drone.yml', rendered)


def build_version_update(version):
    tag_name = generate_tag_name()
    template = read_file(pwd+'/helpers/templates/build.temp')
    rendered = template \
        .replace("%%TAG-NAME%%", tag_name)\
        .replace("%%TAG-VERSION%%", version)
    write_file(pwd+'/helpers/docker-build.sh', rendered)


def dockerfile_image_generate():
    for arch in debian_arches:
        template = read_file(
            pwd+'/helpers/templates/dockerfiles/Dockerfile-'+arch.dpkg+'.temp')
        rendered = template\
            .replace("%%BASE%%", arch.docker)\
            .replace("%%REPO%%", repo_name)\
            .replace("%%RUST-VERSION%%", rust_version)\
            .replace("%%RUSTUP-VERSION%%", rustup_version)\
            .replace("%%CARGO-HOME%%", cargo_home)\
            .replace("%%RUSTUP-HOME%%", rustup_home)\
            .replace("%%PATH%%", path)
        write_file(pwd+'/helpers/docker/Dockerfile-'+arch.dpkg, rendered)


def generate_rust_helper():
    arch_match = 'dpkgArch="$(dpkg --print-architecture)"; \n'
    arch_match += '    case "${dpkgArch##*-}" in \n'
    for arch in debian_arches:
        hash = rustup_hash(arch.rust)
        arch_match += f"        {arch.dpkg}) rustArch='{arch.rust}'; rustupSha256='{hash}' ;; \n"
    arch_match += '        *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \n'
    arch_match += '    esac'

    template = read_file(pwd+"/helpers/templates/add_rust_lang.temp")

    rendered = template \
        .replace("%%RUST-VERSION%%", rust_version) \
        .replace("%%RUSTUP-VERSION%%", rustup_version) \
        .replace("%%RUSTUP-HOME%%", rustup_home) \
        .replace("%%CARGO-HOME%%", cargo_home) \
        .replace("%%PATH%%", path) \
        .replace("%%ARCH-MATCH%%", arch_match)
    write_file(pwd+"/helpers/add_rust_lang.sh", rendered)


def rustup_hash(arch):
    url = f"https://static.rust-lang.org/rustup/archive/{rustup_version}/{arch}/rustup-init.sha256"
    with request.urlopen(url) as f:
        return f.read().decode('utf-8').split()[0]


def generate_image_name(version, arch):
    tag_name = generate_tag_name()
    return tag_name+":"+version+"-"+arch


def read_version():
    return read_file(pwd+"/VERSION")


def generate_tag_name():
    return re.sub(r'\w+://[\w|.]+/', '', repo_name)


def read_file(file):
    with open(file, "r") as f:
        return f.read()


def write_file(file, contents):
    dir = os.path.dirname(file)
    if dir and not os.path.exists(dir):
        os.makedirs(dir)
    with open(file, "w") as f:
        f.write(contents)


if __name__ == '__main__':
    update_versions()
    update_all()
