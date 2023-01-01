# Tackler-rs

This will be rusty [Tackler](https://gitlab.com/e257/accounting/tackler)

## Tackler format and ANTLR parser

See [Parser definition](./src/txn_antlr/readme.adoc) for grammar and lexer.

## Test cases

Simple test-case

    cargo run tests/0001-test.txn

Full format, without unicode characters

    cargo run tests/format-ascii.txn

Full format (doesn't work at the moment)

    cargo run tests/format.txn
