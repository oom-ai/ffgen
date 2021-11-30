# ffgen

[![CICD](https://github.com/oom-ai/ffgen/actions/workflows/CICD.yml/badge.svg)](https://github.com/oom-ai/ffgen/actions/workflows/CICD.yml)
[![license](https://img.shields.io/badge/license-%20MIT/Apache--2.0-blue.svg)](https://github.com/oom-ai/ffgen/releases)
[![crates.io](https://img.shields.io/crates/v/ffgen.svg?colorB=319e8c)](https://crates.io/crates/ffgen)
[![release](https://img.shields.io/badge/Release-%20Linux%20|%20OSX%20|%20Win%20-orange.svg)](https://github.com/oom-ai/ffgen/releases)


A fast **f**ake **f**eature **gen**erator

## Example

```
$ ffgen group account -r fraud_detection.yaml | csview
+------+----------------+--------------+------------------+-------------------+
| user | state          | credit_score | account_age_days | has_2fa_installed |
+------+----------------+--------------+------------------+-------------------+
| 1    | North Carolina | 619          | 1082             | true              |
| 2    | Virginia       | 686          | 596              | true              |
| 3    | Oregon         | 576          | 960              | false             |
| 4    | Nevada         | 540          | 1049             | false             |
| 5    | Massachusetts  | 535          | 229              | true              |
| 6    | West Virginia  | 537          | 462              | false             |
| 7    | New York       | 665          | 156              | false             |
| 8    | Idaho          | 706          | 891              | false             |
| 9    | Arizona        | 667          | 1068             | true              |
| 10   | South Carolina | 526          | 541              | true              |
+------+----------------+--------------+------------------+-------------------+
```

**Integration with oomstore**
```
ffgen schema -r driver_stats.yaml | oomcli apply -f /dev/stdin
2021/11/30 18:55:26 applied

ffgen group account -r fraud_detection.yaml | oomcli import -g account --input-file /dev/stdin
2021/11/30 18:56:31 importing features ...
2021/11/30 18:56:31 succeeded
RevisionID: 1
```

## Installation

### From binaries

Pre-built versions of `ffgen` for various architectures are available at [Github release page](https://github.com/oom-ai/ffgen/releases).

*Note that you can try the `musl` version (which is statically-linked) if runs into dependency related errors.*

### From source

`ffgen` is also published on [crates.io](https://crates.io). If you have Rust toolchains (nightly) installed you can use `cargo` to install it from source:

```
cargo install --locked ffgen
```

If you want the latest version, clone this repository and run `cargo install --path .`.
