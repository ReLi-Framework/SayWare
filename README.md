<!--
SPDX-FileCopyrightText: 2026 The SayWare development team

SPDX-License-Identifier: GPL-3.0-or-later
-->

<div align="center">
  <a href="https://github.com/ReLi-Framework/SayWare/">
    <img src="./assets/images/logo.svg" alt="Logo"/>
  </a>

<h3 align="center">SayWare</h3>

<p align="center">
    A simple and harmless malware that says something
    <br />
    <br />
    <a href="https://github.com/ReLi-Framework/SayWare/">
      <!-- markdownlint-disable-next-line line-length -->
      <img src="https://img.shields.io/badge/GitHub-181717?logo=github&logoColor=fff&style=for-the-badge" alt="Github badge" />
    </a>
    <a href="./LICENSES/GPL-3.0-or-later.txt">
      <!-- markdownlint-disable-next-line line-length -->
      <img src="https://img.shields.io/badge/License-GPL%203.0%20or%20later-green.svg?style=for-the-badge" alt="GPL 3.0 or later badge" />
    </a>
    <a href="https://www.microsoft.com/en-us/windows/">
      <!-- markdownlint-disable-next-line line-length -->
      <img src="https://img.shields.io/badge/Windows-0078D4?logo=windows&logoColor=fff&style=for-the-badge" alt="Windows badge" />
    </a>
    <a href="https://www.rust-lang.org/">
      <!-- markdownlint-disable-next-line line-length -->
      <img src="https://img.shields.io/badge/Rust-000?logo=rust&logoColor=fff&style=for-the-badge" alt="Rust badge" />
    </a>
    <a href="https://reuse.software/">
      <!-- markdownlint-disable-next-line line-length -->
      <img src="https://img.shields.io/reuse/compliance/github.com/ReLi-Framework/SayWare?style=for-the-badge" alt="Reuse badge" />
    </a>
  </p>
</div>

## :clipboard: Table of content

- [:warning: Disclaimer](#warning-disclaimer)
- [:eyes: About the repository](#eyes-about-the-repository)
  - [:question: Why](#question-why)
- [:rocket: Getting started](#rocket-getting-started)
  - [:gear: Prerequisites](#gear-prerequisites)
  - [:hammer_and_wrench: Run the server](#hammer_and_wrench-run-the-server)
  - [:hammer_and_wrench: Build the malware](#hammer_and_wrench-build-the-malware)
- [:construction_worker: Contributing](#construction_worker-contributing)
- [:books: Licenses](#books-licenses)

## :warning: Disclaimer

This malware isn't intended for real use. Because of this, no advanced
techniques will be and have been used. Don't think this is an example of how
real malware is written.

## :eyes: About the repository

This repository contains a harmless malware sample, configurable at build time,
that fetches a sentence from a small server and displays it in a terminal.

### :question: Why

The goal of this repository is to provide a simple, auditable example of a
[ReLi]-compatible repository and of how the provided configuration can be
turned into a concrete executable.

## :rocket: Getting started

Below are steps to run the server and the malware locally.

### :gear: Prerequisites

You need [Cargo] and [rustup] to build the project. The workspace uses the
[nightly][rust nightly] toolchain, and the malware build additionally requires
the `x86_64-pc-windows-msvc` target and the `rust-src` component.

### :hammer_and_wrench: Run the server

The server lets you configure and serve the sentence fetched by the malware.

1. Clone the repository

   ```sh
   git clone https://github.com/ReLi-Framework/SayWare/
   ```

1. Run the server and follow the interactive prompts to choose the endpoint,
   sentence, and optional TLS settings!

   ```sh
   cargo run --release --bin server
   ```

### :hammer_and_wrench: Build the malware

The malware is built from the provided build configuration file.

1. Clone the repository

   ```sh
   git clone https://github.com/ReLi-Framework/SayWare/
   ```

1. Change the [build configuration file] as you like

1. Build the malware!

   ```sh
   cargo build --target x86_64-pc-windows-msvc \
     --bin sayware --profile small-size \
     -Z build-std=core,std,panic_abort \
     -Z build-std-features="optimize_for_size"
   ```

After these steps, the malware will be in the
`./target/x86_64-pc-windows-msvc/small-size/` directory. For cross-compilation,
please refer to the [build workflow].

## :construction_worker: Contributing

Contributions are what make the open source community such an amazing place to
learn, inspire, and create.\
Any contributions you make are **greatly appreciated**.

If you want, you can help me with any kind of work, for example:

- Correct our English errors
- Licensing stuff

## :books: Licenses

Distributed under the [GPL 3.0 or later] license.

[build configuration file]: ./configuration.json
[build workflow]: ./.forgejo/workflows/reli.yaml
[cargo]: https://doc.rust-lang.org/stable/cargo/
[gpl 3.0 or later]: ./LICENSES/GPL-3.0-or-later.txt
[reli]: https://github.com/ReLi-Framework/ReLi/
[rust nightly]: https://doc.rust-lang.org/book/appendix-07-nightly-rust.html
[rustup]: https://rustup.rs/
