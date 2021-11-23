# ffgen

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

## Installation

Pre-built versions of `ffgen` for various architectures are available at [Github release page](https://github.com/oom-ai/ffgen/releases).

*Note that you can try the `musl` version (which is statically-linked) if runs into dependency related errors.*
