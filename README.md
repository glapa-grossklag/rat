# Description

Flip random input bits from `stdin`, send them to `stdout`.

# Usage

```sh
cargo run -- -h
```

The default error rate is 0.1% of bits, but can be changed using the `-e` or
`--error` option. Note that if the error rate is less than 0, no bits will be
flipped. Likewise, if the error rate is greater than 1, all bits will be
fipped.
