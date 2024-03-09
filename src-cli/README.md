# FerroPassGen CLI

[![Crates.io](https://img.shields.io/crates/v/ferropassgen-cli)](https://crates.io/crates/ferropassgen-cli)
[![Downloads](https://img.shields.io/crates/d/ferropassgen-cli.svg)](https://crates.io/crates/ferropassgen-cli)
[![License](https://img.shields.io/crates/l/ferropassgen-cli)](https://crates.io/crates/ferropassgen-cli)
[![Dependency Status](https://deps.rs/repo/github/JamesClarke7283/FerroPassGen/status.svg)](https://deps.rs/repo/github/JamesClarke7283/FerroPassGen)

`ferropassgen-cli` is a command-line interface (CLI) tool for generating strong
and secure passwords and passphrases. It provides a convenient way to create
robust passwords and passphrases directly from the command line.

## Features

- Generate strong passwords with customizable length and character sets
- Generate memorable passphrases with customizable length, separator, and word
  case
- Lightweight and fast password generation
- Easy to use command-line interface

## Usage

To generate a password:

```shell
ferropassgen-cli password --length <LENGTH> --charset <CHARSET>
```

- `--length`: Specify the desired length of the generated password (required).
- `--charset`: Specify the character set to use for password generation
  (required).

Example:

```shell
ferropassgen-cli password --length 16 --charset "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*"
```

To generate a passphrase:

```shell
ferropassgen-cli passphrase --length <LENGTH> [--separator <SEPARATOR>] [--upper]
```

- `--length`: Specify the desired number of words in the generated passphrase
  (required).
- `--separator`: Specify the separator character to use between words in the
  passphrase (optional, default: "-").
- `--upper`: Use uppercase words in the generated passphrase (optional, default:
  lowercase).

Example:

```shell
ferropassgen-cli passphrase --length 4 --separator "_" --upper
```

## Examples

Generate a 20-character password with a custom character set:

```shell
ferropassgen-cli password --length 20 --charset "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
```

Generate a passphrase with 5 words, separated by underscores, and in uppercase:

```shell
ferropassgen-cli passphrase --length 5 --separator "_" --upper
```

Generate a passphrase with 4 words using the default settings (lowercase,
separated by hyphens):

```shell
ferropassgen-cli passphrase --length 4
```

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).

## Acknowledgements

`ferropassgen-cli` is built using the
[ferropassgen](https://crates.io/crates/ferropassgen) library, which provides
the core password generation functionality. Special thanks to the developers of
`ferropassgen` for their excellent work.

The passphrase generation feature uses the
[EFF Long Wordlist](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases)
provided by the [Electronic Frontier Foundation (EFF)](https://www.eff.org/).
