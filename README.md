# Description

Flip random input bits from `stdin`, send them to `stdout`.

# Installation

Install with:

```sh
$ cargo install --path .
```

and uninstall with:

```sh
$ cargo uninstall
```

# Usage


The program will read from `stdin` and write to `stdout`, flipping a percentage
of bits. View all options with `$ rat -h`.

The default error rate is 0.1% of bits, but can be changed using the `-e` or
`--error` option. Note that if the error rate is less than 0, no bits will be
flipped. Likewise, if the error rate is greater than 1, all bits will be
fipped.

To view how many bits were flipped and the total amount of bytes processed, run
with the `-v` or `--verbose` option.

# Examples

```sh
$ printf 'a%.0s' {1..100} | rat -v -e 0.005
Flipped: 3 bits
Written: 100 bytes
aa�aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa`aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaeaaaaaaaaaaaaa
```
The example above runs one hundred 'a's through rat in verbose mode (`-v`) with
an error rate of 0.5% (`-v 0.005`). Three bits of the one hundred bytes were
flipped.
