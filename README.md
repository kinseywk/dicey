# Dicey

A simple library for parsing dice strings (e.g., 1d20, 2d12, 3d8) you might see in tabletop games like Dungeons & Dragons.

Provides a single struct, DieRoll, and single function, parse(&str) that converts an input string to a list of DieRolls.

Usage:

```rust
use dicey::parse;
use rand::*;

fn main() {
	if let Some(mut rolls) = parse("5d6") {
		if let Some(roll) = rolls.pop() {
			for _ in 0..roll.quantity {
				println!("{}", random::<u8>() % roll.faces + 1);
			}
		}
	}
}
```

Also contains the _dicey_ command line utility for simulating dice rolls from dice strings.

Examples:

* Coin flip (heads = 1, tails = 2): dicey 1d2

* Yahtzee: dicey 5d6

* D&D initiative, saving throws, etc.: dicey 1d20

* BattleTech multiple weapons: dicey 2d6,2d6,2d6