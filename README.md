# Arithmetic Coder in Rust

An arithmetic coder written in Rust. Ported from the **C** implementation
provided
[here](https://web.stanford.edu/class/ee398a/handouts/papers/WittenACM87ArithmCoding.pdf).

## Building

The project comes with an encoder binary, `encode`, and a decoder binary
`decode`. To build both:

```bash
$ cargo build
```

To build just the encoder:

```bash
$ cargo build --bin encode
```

To build just the decoder:

```bash
$ cargo build --bin decode
```

## Usage

Both the `encode` and `decode` binaries read from `stdin` and write to `stdout`.
An example pipeline using the two:

```bash
$ cat README.md | ./encode | ./decode
```
