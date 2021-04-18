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
of bits.

```
$ rat --help
Usage:
    rat [options]

Options:
    -h, --help          Print this information.
    -v, --verbose       Print how many bits were flipped and how many bytes
                        were written.
    -e, --error RATE    The rate at which to flip bits, default is 0.001.
    -s, --seed SEED     The seed for the random number generator if
                        reproducable results are needed. Must be an unsigned
                        integer.
```

Note that if the error rate is less than 0, no bits will be flipped. Likewise,
if the error rate is greater than 1, all bits will be fipped.


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
