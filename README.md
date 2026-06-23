# rust-password-generator

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Security](https://img.shields.io/badge/Security-OsRng-orange?style=for-the-badge&logo=shield&logoColor=white)
![CLI](https://img.shields.io/badge/CLI-Terminal-blue?style=for-the-badge&logo=windowsterminal&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)

A password generator I built in Rust. I've been seeing Rust everywhere lately —
in security tools, system utilities, even the Linux kernel — and wanted to
actually try it instead of just reading about it. I'm also genuinely fascinated
by how fast Rust is. No garbage collector, just raw speed with memory safety enforced at compile time.

## What it does

- Generates a random password from the full printable ASCII charset
- Uses `OsRng` for cryptographically secure randomness — not just `rand::thread_rng()`
- Rates the strength of the generated password based on length
- Automatically copies the password to your clipboard so you don't have to

## Usage

You need Rust installed: https://rustup.rs

```bash
git clone https://github.com/888mika888/rust-password-generator
cd rust-password-generator
cargo run
```

Type in a length when it asks, and it handles the rest.

## Strength ratings

| Length | Rating      |
|--------|-------------|
| 0–6    | Weak        |
| 7–11   | Medium      |
| 12–17  | Strong      |
| 18+    | Very Strong |

Use 16+ for anything you actually care about.

## Character set

abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!#$%&'()*+,-./:;<=>?@[]^_`{|}~

Full printable ASCII — lowercase, uppercase, digits, and every standard symbol.

## Dependencies

- [rand](https://crates.io/crates/rand) `0.8` — RNG, specifically `OsRng`
- [arboard](https://crates.io/crates/arboard) `3` — clipboard access, works on Windows, Linux and macOS

## Why Rust

I wanted to actually learn Rust instead of just hearing about it. It has a
reputation for being as fast as C while being way safer — the compiler catches
memory issues before your code even runs. That kind of thing interests me a lot,
especially coming from PHP and Java where you don't think about memory at all.
This project taught me ownership, pattern matching, iterators, and how to pull
in external crates. Small project but it clicked a lot of things into place.

## What I want to add

- [ ] Generate multiple passwords at once
- [ ] Option to save passwords to a file
