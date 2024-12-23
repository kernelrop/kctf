# KCTF Solver

A fast asynchronous library and cli to solve (or generate) proof-of-work challenges generated using the kctf scheme.

## Installation

```sh
cargo install kctf
```

This will install `kctf` to your cargo binary directory, usually at `~/.cargo/bin`

## Challenge Usage

This program can be used in your challenges as a drop in replacement for the [kCTF script](https://github.com/google/kctf/blob/v1/docker-images/challenge/pow.py)

## CLI Usage

```sh
## Solve a single challenge
## Expected output: s.LR15WHZE5YO/8EEY9BF7pdvxiJxwkDi7mdS52bg7eVUdHbAwBVxfahl/qxceccZV2PHkj4wQTQ9Ng837/KD9IWQL4v2GmRyjc5O9MxiAXBtxn7FYjjA2as/17lF2lEtQtABbSEUgxam+sIsdfDJMAUzn4fYsS7vOarXh7iY6ZYknrwt1S8EHyQeYkoTUzkpUIVAuSvl8jExcPzvmuaoM6A==
kctf solve s.AAU5.AACV7mM375HM8wElUbxsknqD

## Ask a single challenge
## If the correct solution is given kctf will exit with status code of 0
kctf ask

## Generate a single challenge of specified difficulty
## This shouldn't be used if you are using kctf for pow, instead you should use ask
kctf gen 50

## Verify a single challenge
## This is not part of the official implementation, but it's good to have it
## kctf <challenge> <solution>
kctf verify s.AAU5.AACV7mM375HM8wElUbxsknqD s.LR15WHZE5YO/8EEY9BF7pdvxiJxwkDi7mdS52bg7eVUdHbAwBVxfahl/qxceccZV2PHkj4wQTQ9Ng837/KD9IWQL4v2GmRyjc5O9MxiAXBtxn7FYjjA2as/17lF2lEtQtABbSEUgxam+sIsdfDJMAUzn4fYsS7vOarXh7iY6ZYknrwt1S8EHyQeYkoTUzkpUIVAuSvl8jExcPzvmuaoM6A==
```

## Benchmarks

The performance of kctf vs kctf-pow is nearly identical. However, if you require async support, or require more precise errors, you should use kctf.

Benchmarks can be done using the `cargo bench` command.
