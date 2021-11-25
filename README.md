# ffgen

[![CICD](https://github.com/oom-ai/ffgen/actions/workflows/CICD.yml/badge.svg)](https://github.com/oom-ai/ffgen/actions/workflows/CICD.yml)
[![license](https://img.shields.io/badge/license-%20MIT/Apache--2.0-blue.svg)](https://github.com/oom-ai/ffgen/releases)
[![release](https://img.shields.io/badge/Release-%20Linux%20|%20OSX%20|%20Win%20-orange.svg)](https://github.com/oom-ai/ffgen/releases)


A fast **f**ake **f**eature **gen**erator

## Example

```
$ ffgen list group
fraud_detection_account
fraud_detection_transaction_stats
```

```
$ ffgen group fraud_detection_account --id-range 1..10 | csview
+------+----------------+--------------+------------------+-------------------+
| user | state          | credit_score | account_age_days | has_2fa_installed |
+------+----------------+--------------+------------------+-------------------+
| 1    | New Mexico     | 701          | 69               | true              |
| 2    | Tennessee      | 575          | 359              | false             |
| 3    | Massachusetts  | 637          | 33               | false             |
| 4    | West Virginia  | 608          | 359              | true              |
| 5    | Alaska         | 687          | 225              | false             |
| 6    | North Carolina | 665          | 37               | true              |
| 7    | Idaho          | 698          | 261              | false             |
| 8    | Texas          | 544          | 299              | true              |
| 9    | Minnesota      | 640          | 361              | true              |
| 10   | Minnesota      | 731          | 330              | true              |
+------+----------------+--------------+------------------+-------------------+
```

**Integration with oomctl**
```
$ ffgen schema fraud_detection | oomctl apply -f /dev/stdin
2021/11/24 16:23:01 applied

$ ffgen group fraud_detection_account | oomctl import -g account --input-file /dev/stdin
2021/11/24 16:23:15 succeeded
RevisionID: 5
```

## Installation

Pre-built versions of `ffgen` for various architectures are available at [Github release page](https://github.com/oom-ai/ffgen/releases).

*Note that you can try the `musl` version (which is statically-linked) if runs into dependency related errors.*
