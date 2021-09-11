#![allow(non_snake_case, non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DieRoll {
	pub quantity: usize,
	pub faces: usize,
}

pub fn parse(diceRoll: &str) -> Option<Vec<DieRoll>> {
	let mut result: Vec<DieRoll> = Vec::new();

	//1.) Split input string on each ","
	let diceRoll: Vec<&str> = diceRoll.split(',').collect();

	//2.) Split each of those strings on each "d"
	for dieRoll in diceRoll {
		let dieRoll: Vec<&str> = dieRoll.split('d').collect();

		if dieRoll.len() != 2 {
			return None;
		}

		//3.) Parse each of _those_ strings as positive integers and create a DieRoll from each pair
		// if let (quantity, faces) = (dieRoll[0]::parse::<usize>(), dieRoll[1]::parse::<usize>() {
		if let (Ok(quantity), Ok(faces)) = (dieRoll[0].parse::<usize>(), dieRoll[1].parse::<usize>()) {
			if quantity == 0 || faces == 0 {
				return None;
			} else {
				result.push(DieRoll{quantity, faces});
			}
		} else {
			return None;
		}
	}

	Some(result)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_single() {
		const test: &str = "5d6";

		if let Some(result) = parse(test) {
			assert_eq!(result[0], DieRoll{quantity: 5, faces: 6});
		} else {
			panic!("Failed to parse string \"{}\"", test);
		}
	}

	#[test]
	fn parse_list() {
		const test: &str = "1d1,100d100,1000000d1000000";

		if let Some(result) = parse(test) {
			assert_eq!(result.len(), 3, "Failed to parse all list entries");
			assert_eq!(result[0], DieRoll{quantity: 1, faces: 1});
			assert_eq!(result[1], DieRoll{quantity: 100, faces: 100});
			assert_eq!(result[2], DieRoll{quantity: 1000000, faces: 1000000});
		} else {
			panic!("Failed to parse string \"{}\"", test);
		}
	}

	#[test]
	fn parse_fail() {
		const tests: &'static [&'static str] = &["r2d2", "-3d3", "3d-3", "3d0", "0d3", "3d", "d3", ""];

		for test in tests {
			assert_eq!(parse(test), None, "Incorrectly parsed invalid string \"{}\"", test);
		}
	}
}