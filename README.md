# rust-cli\_calculator

[![Build Status](
https://travis-ci.org/benaryorg/rust-cli_calculator.svg?branch=master
)](https://travis-ci.org/benaryorg/rust-cli_calculator)

This is a small command line calculator taking command line arguments describing
some math things, e.g. `5*3*(4+1*5.123/3+3/5+(3*3)*(123-3))` returning the
correct result e.g. `16294.615000000002`.

Also, all calculations are stored in strings and are therefore not very
accurate.
There are currently no plans to rewrite this, though it would be quite
interesting to solve this with an enum of operations.

# Building & Running

    $ cargo build --release
	$ cargo run --release -- '<calculation>'

You then have the binary.
You know what to do with it, I'm sure.

# Contribution

Yes.
Please.
Just a PR is enough.

There are already TODOs in the comments.

# License

MIT.

