# dtl-parser
A "parser" for Sesam's DTL (Data Transformation Language) written in Rust

## Development requirements
* pre-commit

Then run the following in the root of the project:
```
$ pre-commit install
```
After this is done it will run the following on each commit:
```
cargo check
cargo fmt
cargo clippy
```
Checks formatting, runs the tests, all that good stuff.

