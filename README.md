# EsbFireplace - Rust

The FIREPLACEv1.0 allows the use of the `esb` tooling for solving Advent of Code problems.
This is an implementation of FIREPLACEv1.0 for [rust](https://www.rust-lang.org/).

Check [esb](https://github.com/luxedo/esb) for more information.

## Installation

The package can be installed by adding `esb_fireplace` to your list of dependencies in `Cargo.toml`:

```toml
[dependencies]
esb_fireplace = { version = "0.1.0" }
```

## Usage

Create a function named `start` in your solution file and add `EsbFireplace.v1_run` to it.

```rust
use std::error::Error;

use esb_fireplace::Fireplace;

fn solve_pt1(input_data: &str, _args: Option<Vec<String>>) -> Result<i32, Box<dyn Error>> {
    Ok(10)
}

fn solve_pt2(input_data: &str, _args: Option<Vec<String>>) -> Result<String, Box<dyn Error>> {
    Ok("hello")
}

fn main() -> Result<(), Box<dyn Error>> {
    // 🎅🎄❄️☃️🎁🦌
    // Bright christmas lights HERE
    Fireplace::v1_run(solve_pt1, solve_pt2)
}
```

Running can be done with `cargo`, but this library is meant to be used with [esb](https://github.com/luxedo/esb).

```bash
# You can do this...
cargo run -- --part 1 < input_data.txt

# But instead do this:
esb run --year 2023 --day 1 --lang rust --part 1
```

The docs can be found at <https://hexdocs.pm/esb_fireplace>.

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
