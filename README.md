# Flask Unsign Rust

This repo is inspired by [Flask Unsign](https://github.com/Paradoxis/Flask-Unsign). The original repo is written in Python,  I decided to write a Rust version of it. Decodes, brute-forces, and generates Flask session cookies.

## Build

```bash
cargo build --release
```

## Usage

There are 3 subcommands in this tool: decode, sign, unsign. To get help:

```bash
flask-unsign-rust --help
```

### Decode

Decode a Flask session cookie.

```bash
flask-rs decode --cookie "eyJsb2dnZWRfaW4iOnRydWV9.Y-6iDg.s4WvxNEtKHvjSIXmG9Zx8YYcci8"
```

### Sign

```bash
target/debug/flask-rs sign --payload='{"logged_in":true}' --secret="CHANGEME"
```

### Unsign

(This feature is not implemented yet)

This brute-forces the secret key. It will try all the words in the wordlist.

```bash
flask-unsign-rust unsign --cookie "eyJhZG1pbiI6ZmFsc2UsInVpZCI6ImNlbmVrc2FuemFrIn0.Y8LELg.Be7MYQQSD-rm0xm4XGDk6IJ4aWQ" --wordlist /usr/share/wordlists/rockyou.txt
```
