Not actually a parser, I just don't know what to call it :3
It's basically a DTL translation layer for Feldera.

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

