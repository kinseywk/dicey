extern crate dicey;

use std::env;
use dicey::parse;
use rand::Rng;

fn main() {
	let mut args: Vec<String> = env::args().collect();
	args.remove(0);

	let mut rng = rand::thread_rng();

	for arg in args {
		let mut sum: isize = 0;
		let mut adjustment: isize = 0;

		#[cfg(debug_assertions)]
		println!("Input string \"{}\"", arg);
		
		if let Some(rolls) = parse(&arg) {
			for roll in rolls {
				#[cfg(debug_assertions)]
				println!("Parsed as {:?}", roll);

				println!("Rolling {}d{}:", roll.quantity, roll.faces);

				for n in 0..roll.quantity {
					//For each DieRoll.quantity, obtain a random integer in range [1, DieRoll.faces]
					let face = rng.gen::<usize>() % roll.faces + 1;
					println!("#{}: {}", n + 1, face);
					sum += face as isize;
				}

				adjustment += roll.adjustment;
			}
		} else {
			println!("Parsing failure");
		}
		
		if adjustment == 0 {
			println!("Total = {}\n", sum);
		} else if adjustment > 0 {
			println!("Total = {} from dice + {} from bonus = {}\n", sum, adjustment, sum + adjustment);
		} else if adjustment < 0 {
			println!("Total = {} from dice - {} from malus = {}\n", sum, -adjustment, sum + adjustment);
		}
	}
}