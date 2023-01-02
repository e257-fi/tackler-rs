# Tackler-rs

This will be rusty [Tackler](https://gitlab.com/e257/accounting/tackler)

## POC of Tackler grammar with ANTLR parser generator for rust target

See [Parser definition](./src/txn_antlr/readme.adoc) for grammar and lexer.

The rust target for ANTLR is here: https://github.com/rrevenantt/antlr4rust


## Test cases

Simple test-case

    cargo run tests/0001-test.txn

Journal with unicode high-bit UTF-8 characters

    cargo run tests/utf8-journal.txn

Full format

    cargo run tests/format.txn

## Error cases (e.g. invalid input)

Simple invalid input with malformed date

    cargo run tests/err-0001.txn

Invalid input with duplicate metadata elements

    cargo run tests/err-0002.txn

