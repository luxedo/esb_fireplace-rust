# EsbFireplace - Rust

The [FIREPLACEv1.0 protocol](https://github.com/luxedo/esb/blob/main/doc/FIREPLACEv1.0.md)
allows the use of the `esb` tooling for solving Advent of Code problems.
This is an implementation of FIREPLACEv1.0 for [rust](https://www.rust-lang.org/).

Check [esb](https://github.com/luxedo/esb) for more information.

## Installation

The package can be installed by adding `esb_fireplace` to your list of dependencies in `Cargo.toml`:

```toml
[dependencies]
esb_fireplace = { version = "0.3.0" }
```

## Usage

Create a function named `start` in your solution file and add `EsbFireplace.v1_run` to it.

```rust
use esb_fireplace::{FireplaceError, FireplaceResult};

use std::fmt::Display;

fn solve_pt1(input_data: &str, _args: Vec<String>) -> FireplaceResult<impl Display> {
    Ok(25)
}

fn solve_pt2(input_data: &str, _args: Vec<String>) -> FireplaceResult<impl Display> {
    Ok("December")
}

fn main() -> Result<(), FireplaceError> {
    // 🎅🎄❄️☃️🎁🦌
    // Bright christmas lights HERE
    esb_fireplace::v1_run(solve_pt1, solve_pt2)
}
```

Running can be done with `cargo`, but this library is meant to be used with [esb](https://github.com/luxedo/esb).

```bash
# You can do this...
cargo run -- --part 1 < input_data.txt

# But instead do this:
esb run --year 2023 --day 1 --lang rust --part 1
```

The docs can be found at <https://docs.rs/esb_fireplace/>.

## Contributors

We want to acknowledge and thank the following contributors for their efforts in making this project better:

- [gustavobat](https://github.com/gustavobat)

Thank you all for your valuable contributions!

## License

> ESB - Script your way to rescue Christmas as part of the ElfScript Brigade team.
> Copyright (C) 2024 Luiz Eduardo Amaral <luizamaral306@gmail.com>
>
> This program is free software: you can redistribute it and/or modify
> it under the terms of the GNU General Public License as published by
> the Free Software Foundation, either version 3 of the License, or
> (at your option) any later version.
> This program is distributed in the hope that it will be useful,
> but WITHOUT ANY WARRANTY; without even the implied warranty of
> MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
> GNU General Public License for more details.
> You should have received a copy of the GNU General Public License
> along with this program. If not, see <http://www.gnu.org/licenses/>.
