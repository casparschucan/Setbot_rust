pub mod set{
	extern crate rand;
    use rand::seq::SliceRandom;
	use rand::thread_rng;
	use svg::node::element::SVG;
	use svg::node::element::Path;
	use svg::node::element::path::Data;
	use svg::Document;

	use svg::node::element::Rectangle;
	use svg::node::element::Ellipse;
	use std::cmp;

	pub struct Game {
		pub cur_cards: Vec<(usize, usize, usize, usize)>,
		stack: Vec<(usize, usize, usize, usize)>,
		pub ended: bool
	}

	impl Game {
		pub fn guess(& mut self, card1: usize, card2: usize, card3: usize) {
			if is_set(&self.cur_cards, &(card1, card2, card3)) {
				println!("correct");
				if self.stack.len() >= 3 {
					self.cur_cards[card1] = self.stack.pop().unwrap_or((0, 0, 0 ,0));
					self.cur_cards[card2] = self.stack.pop().unwrap_or((0, 0, 0 ,0));
					self.cur_cards[card3] = self.stack.pop().unwrap_or((0, 0, 0 ,0));
					if !contains_set(&self.cur_cards){
						self.add_cards();
					}
					save_to_svg(draw_deck(&self.cur_cards), self.cur_cards.len()/3*100, 480);
				} else if contains_set(&self.cur_cards) {
					println!("{:?}", self.cur_cards);
					save_to_svg(draw_deck(&self.cur_cards), self.cur_cards.len()/3*100, 480);
				} else {
					self.ended = true;
					println!("The game has ended");
				}
				
			} else {
				println!("wrong");
			}
		}

		pub fn add_cards(& mut self) {
			if self.stack.len() >= 3 {
				self.cur_cards.push(self.stack.pop().unwrap_or((0, 0, 0, 0)));
				self.cur_cards.push(self.stack.pop().unwrap_or((0, 0, 0, 0)));
				self.cur_cards.push(self.stack.pop().unwrap_or((0, 0, 0, 0)));
			}
			save_to_svg(draw_deck(&self.cur_cards), self.cur_cards.len()/3*100, 480)
		}
	}

	pub fn new_game() -> Game {
		let mut game = Game {
			cur_cards: vec![],
			stack: vec![],
			ended: false
		};
		game.stack.reserve(81);
		for i in 0..81 {
			game.stack.push((i % 3, (i / 3) % 3, (i / 9) % 3, (i / 27) % 3));
		}
		game.stack.shuffle(& mut thread_rng());
		for _ in 0..12 {
			game.cur_cards.push(game.stack.pop().unwrap());
		}
		save_to_svg(draw_deck(&game.cur_cards), game.cur_cards.len()/3*100, 480);
		println!("{:?}", game.cur_cards);
		game
	}
	fn is_set(cards: &Vec<(usize, usize, usize, usize)>, guess: &(usize, usize, usize)) -> bool {
		if cmp::max(guess.0, cmp::max(guess.1, guess.2)) >= cards.len() {
			return false;
		}

		let card1 = cards[guess.0];
		let card2 = cards[guess.1];
		let card3 = cards[guess.2];

		
		if guess.0 == guess.1 || guess.1 == guess.2 || guess.0 == guess.2 {
			return false;
		}
		if (card1.0 + card2.0 + card3.0) % 3 != 0 {
			return false;
		}
		if (card1.1 + card2.1 + card3.1) % 3 != 0 {
			return false;
		}
		if (card1.2 + card2.2 + card3.2) % 3 != 0 {
			return false;
		}
		if (card1.3 + card2.3 + card3.3) % 3 != 0 {
			return false;
		}
		true
	}

	fn contains_set(cards: &Vec<(usize, usize, usize, usize)>) -> bool {
		match find_set(cards) {
			None => false,
			Some(_) => true
		}
	}
	fn find_set(cards: &Vec<(usize, usize, usize, usize)>) -> Option<(usize, usize, usize)> {
		for i in 0..12 {
			for j in i+1..12 {
				for k in j+1..12 {
					if is_set(cards, &(i, j, k)) {
						println!("[{}, {}, {}]", i, j, k);
						return Some((i, j, k));
					}
				}
			}
		}
		return None;
	}

	fn draw_deck(cards: &Vec<(usize, usize, usize, usize)>) -> Vec<SVG> {
		let mut drawn_cards: Vec<SVG> = vec![];
		for i in 0..cards.len() {
			let rect = Rectangle::new().set("x", 0)
				.set("y", 0)
				.set("stroke", "black")
				.set("stroke-width", 3)
				.set("fill", "white")
				.set("height", 100)
				.set("width", 160);
			let card = SVG::new()
				.set("x", (i % 3) * 160)
				.set("y", (i / 3) * 100)
				.set("height", 100)
				.set("width", 160)
				.add(rect)
				.add(draw_card(&cards[i]));
			drawn_cards.push(card);
		}
		drawn_cards
	}

	fn draw_card(card: &(usize, usize, usize, usize)) -> SVG {
		let color = ["blue", "red", "green"];
		let opacity = ["100%", "50%", "0%"];
		let items_count = card.0 + 2;
		let mut drawn_card: SVG = SVG::new();
		for i in 1..items_count {
			let rom_dat = Data::new()
				.move_to((i*160/items_count, 15))
				.line_by((15, 35))
				.line_by((-15, 35))
				.line_by((-15, -35))
				.line_by((15, -35))
				.close();
			if card.1 == 0 {
				let rect = Rectangle::new()
					.set("x", i*160/items_count - 20)
					.set("y", 15)
					.set("width", 30)
					.set("height", 70)
					.set("stroke", color[card.2])
					.set("stroke-width", 1)
					.set("fill", color[card.2])
					.set("fill-opacity", opacity[card.3]);
				drawn_card = drawn_card.add(rect)
			}
			if card.1 == 1 {
				
				let rom: Path = Path::new()
					.set("stroke", color[card.2])
					.set("stroke-width", 1)
					.set("fill", color[card.2])
					.set("fill-opacity", opacity[card.3])
					.set("d", rom_dat);
				drawn_card = drawn_card.add(rom);
			}
			if card.1 == 2 {
				let ell = Ellipse::new()
					.set("cx", i*160/items_count)
					.set("cy", 50)
					.set("rx", 15)
					.set("ry", 35)
					.set("stroke", color[card.2])
					.set("stroke-width", 1)
					.set("fill", color[card.2])
					.set("fill-opacity", opacity[card.3]);
				drawn_card = drawn_card.add(ell);
			}
		}
		drawn_card
	}

	fn save_to_svg(drawn_cards: Vec<SVG>, height: usize, width: usize) {
		let mut doc = Document::new()
			.set("height", height)
			.set("width", width);
		
		for card in drawn_cards {
			doc = doc.add(card);
		}
		svg::save("image.svg", &doc).unwrap();
	}

	pub enum Input {
		Command(String),
		Guess(Vec<usize>)
	}
	pub fn parse_input(str_guess: & mut String) -> Option<Input> {	
		return match str_guess.as_str().to_lowercase().trim() {
			"add cards" => Some(Input::Command(str_guess.to_string())),
			_ => parse_guess(&str_guess),
		}
	}
	fn parse_guess(str_guess: &String) -> Option<Input> {
		let guess: Vec<usize> = str_guess.split_whitespace()
									  .map(|x| x.parse().expect("invalid input format couldn't parse to int"))
									  .collect();
		return Some(Input::Guess(guess));
	}
}

#[cfg(test)]
mod test {

	use super::set::*;
	#[test]
	#[should_panic]
	fn test_parse_invalid_input() {
		let mut str_guess = String::from("addd cards");
		parse_input(& mut str_guess);
	}
	#[test]
	fn test_parse_valid_input() {
		let mut command = String::from("add cards");
		let mut command_upper = String::from("AdD CaRdS");
		let mut numbers = String::from("0 4 18");
		let parsed_command = parse_input(& mut command);
		let parsed_command_upper = parse_input(& mut command_upper);
		let parsed_numbers = parse_input(& mut numbers);
		
		assert!(matches!(parsed_command, Some(_)));
		match parsed_command {
			Some(input) => assert!(matches!(input, Input::Command(_))),
			None => {}
		}

		assert!(matches!(parsed_command_upper, Some(_)));
		match parsed_command_upper {
			Some(input) => assert!(matches!(input, Input::Command(_))),
			None => {}
		}

		assert!(matches!(parsed_numbers, Some(_)));
		match parsed_numbers {
			Some(input) => {
				assert!(matches!(input, Input::Guess(_)));
				match input {
					Input::Guess(vec) => {
						assert_eq!(vec, Vec::from([0, 4, 18]));
					}
					Input::Command(_) => {}
				}
			}
			None => {}
		}
	}
}