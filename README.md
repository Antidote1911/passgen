[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/Antidote1911/passgen/blob/master/LICENSE-MIT)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/Antidote1911/passgen/blob/master/LICENSE-APACHE)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
[![CI](https://github.com/Antidote1911/passgen/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/Antidote1911/passgen/actions/workflows/ci.yml)
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

Generate a serial number with 4 digits x3 and save in txt file ( xxxx-xxxx-xxxx ) :
```
./passgen -d -c 3 -L 4 --output mypasswords.txt
4693-8883-4528
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
    -L, --length <NUMBER>     Sets the required length [default: 12]
    -E, --entropy <NUMBER>    Sets the minimum required entropy (conflicts with --length)
    -c, --count <NUMBER>      Number of section in serial style [default: 1]
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
