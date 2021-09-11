#![allow(non_snake_case, non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DieRoll {
	//Number of dice in the roll
	pub quantity: usize,
	//Number of die faces (6 = standard cubic die)
	pub faces: usize,
	//Net bonus and malus applied to the roll (e.g., 1d4+1 adds 1 to each roll)
	pub adjustment: isize,
}

pub fn parse(diceRoll: &str) -> Option<Vec<DieRoll>> {
	let mut result: Vec<DieRoll> = Vec::new();

	//1.) Split input string on each ','
	let diceRoll: Vec<&str> = diceRoll.split(',').collect();

	for dieRoll in diceRoll {
		let mut adjustmentChar: Option<char> = None;
		let mut adjustment: isize = 0;
		let mut dieString: &str = dieRoll;

		//2.) Split-off the '+' or '-' clause, if one exists
		//2a.) Test if '+' and '-' are in the string
		if let Some(_) = dieRoll.find('+') {
			adjustmentChar = Some('+');
		} else if let Some(_) = dieRoll.find('-') {
			adjustmentChar = Some('-');
		}

		//2b.) If either exists, break-off the front part and parse the latter part as an integer
		if let None = adjustmentChar {
		} else {
			if let Some((diePart, adjustmentPart)) = dieString.split_once(adjustmentChar.unwrap()) {
				dieString = diePart;

				if let Ok(adjustmentParsed) = adjustmentPart.parse::<isize>() {
					adjustment = adjustmentParsed;

					//2c.) Finally, if the prior test matched a '-', make the parsed number negative
					if adjustmentChar.unwrap() == '-' {
						adjustment = -adjustment;
					}
				} else {
					return None;
				}
			}
		}

		//3.) Split each dice string on 'd'
		if let Some((quantityPart, facesPart)) = dieString.split_once('d') {
			//4.) Parse each of _those_ strings as positive integers and create a DieRoll from each pair
			if let (Ok(quantity), Ok(faces)) = (quantityPart.parse::<usize>(), facesPart.parse::<usize>()) {
				if quantity == 0 || faces == 0 {
					return None;
				} else {
					result.push(DieRoll{quantity, faces, adjustment});
				}
			} else {
				return None;
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
		{
			const test: &str = "5d6";

			if let Some(result) = parse(test) {
				assert_eq!(result[0], DieRoll{quantity: 5, faces: 6, adjustment: 0});
			} else {
				panic!("Failed to parse string \"{}\"", test);
			}
		}

		{
			const test: &str = "3d4+1";

			if let Some(result) = parse(test) {
				assert_eq!(result[0], DieRoll{quantity: 3, faces: 4, adjustment: 1});
			} else {
				panic!("Failed to parse string \"{}\"", test);
			}
		}

		{
			const test: &str = "1d2-1";
	
			if let Some(result) = parse(test) {
				assert_eq!(result[0], DieRoll{quantity: 1, faces: 2, adjustment: -1});
			} else {
				panic!("Failed to parse string \"{}\"", test);
			}
		}
	}

	#[test]
	fn parse_list() {
		const test: &str = "1d1,100d100,1000000d1000000,1d100+100,100d1-1";

		if let Some(result) = parse(test) {
			assert_eq!(result.len(), 5, "Failed to parse all list entries");
			assert_eq!(result[0], DieRoll{quantity: 1, faces: 1, adjustment: 0});
			assert_eq!(result[1], DieRoll{quantity: 100, faces: 100, adjustment: 0});
			assert_eq!(result[2], DieRoll{quantity: 1000000, faces: 1000000, adjustment: 0});
			assert_eq!(result[3], DieRoll{quantity: 1, faces: 100, adjustment: 100});
			assert_eq!(result[4], DieRoll{quantity: 100, faces: 1, adjustment: -1});
		} else {
			panic!("Failed to parse string \"{}\"", test);
		}
	}

	#[test]
	fn parse_fail() {
		const tests: &'static [&'static str] = &["r2d2", "-3d3", "3d-3", "3d0", "0d3", "3d", "d3", "3d3,", ",", ""];

		for test in tests {
			assert_eq!(parse(test), None, "Incorrectly parsed invalid string \"{}\"", test);
		}
	}
}