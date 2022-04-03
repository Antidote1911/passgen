[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/Antidote1911/passgen/blob/master/LICENSE-MIT)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/Antidote1911/passgen/blob/master/LICENSE-APACHE)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
[![Build Status](https://img.shields.io/github/workflow/status/Antidote1911/passgen/Cargo%20Build%20&%20Test?style=flat-square)](https://github.com/Antidote1911/passgen/actions/workflows/ci.yml?query=branch%3Amaster)

# 🔑 Passgen

A simple cli password generator.

## Usage examples

 With no argument Passgen generate a password with 12 chars. It use uppercase letters, lowercase letters and digits:
```
./passgen
POvzXaq3CDoj
```

Generate a password with 30 chars with upper,lower,digits and symbols:
```
./passgen -ulds -L 30
K~n&n&ilbafGqtra&^v&Tal2ttnxD7
```

Generate 3 passwords with 15 chars with upper and digits and save in txt file:
```
./passgen -ud -c 3 -L 15 --output mypasswords.txt
8DC2HM510IJ9LXO
68AUQ7YN6V8JYJG
NZGBEHVCJ6MPDWW
File 'mypasswords.txt' is saved.
```
Display full help with -h flag:

```
./passgen -h

passgen 0.4.0
Antidote1911 <antidote1911@gmail.com>
🔑 Random password generator

USAGE:
    passgen [OPTIONS]

OPTIONS:
    -u, --uppercase           Use UPPERCASE letters [A-Z]
    -l, --lowercase           Use lowercase letters [a-z]
    -d, --digits              Use digits [0-9]
    -s, --symbols             Use special symbols [*&^%$#@!~]
    -o, --others              Use other symbols [♕♖♗♘♙♚...]
    -L, --length <NUMBER>     Sets the required password length [default: 12]
    -E, --entropy <NUMBER>    Sets the minimum required password entropy (conflicts with --length)
    -c, --count <NUMBER>      Number of passwords [default: 1]
    -i, --info                Prints password information
        --output <OUTPUT>     Output in a txt file
        --config              Sets config to default values
    -h, --help                Print help information
    -V, --version             Print version information

If you do not specify any of the [--uppercase, --lowercase, --digits, --symbols, --others] flags,
then uppercase, lowercase letters and digits will be used.
```
## Build Passgen
Clone this repo, go in passgen folder and build it with cargo :
```
git clone https://github.com/Antidote1911/passgen
cd passgen
cargo build --release

```
