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
	use std::io;

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
					save_to_svg(draw_deck(&self.cur_cards));
				} else if contains_set(&self.cur_cards) {
					println!("{:?}", self.cur_cards);
					save_to_svg(draw_deck(&self.cur_cards));
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
		save_to_svg(draw_deck(&game.cur_cards));
		println!("{:?}", game.cur_cards);
		game
	}
	pub fn add_cards() {}
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

	pub fn draw_deck(cards: &Vec<(usize, usize, usize, usize)>) -> Vec<SVG> {
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

	pub fn draw_card(card: &(usize, usize, usize, usize)) -> SVG {
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

	pub fn save_to_svg(drawn_cards: Vec<SVG>) {
		let mut doc = Document::new()
			.set("height", 400)
			.set("width", 480);
		
		for card in drawn_cards {
			doc = doc.add(card);
		}
		svg::save("image.svg", &doc).unwrap();
	}

	pub enum Input {
		Command(String),
		Guess(Vec<usize>)
	}
	pub fn parse_input() -> Option<Input> {
		let mut str_guess = String::new();
		let stdin = io::stdin();
		stdin.read_line(& mut str_guess).expect("invalid input couldn't read line");	
		return match str_guess.as_str() {
			"add cards" => Some(Input::Command(str_guess)),
			_ => parse_guess(&str_guess),
		}
	}
	pub fn parse_guess(str_guess: &String) -> Option<Input> {
		let guess: Vec<usize> = str_guess.split_whitespace()
									  .map(|x| x.parse().expect("invalid input format couldn't parse to int"))
									  .collect();
		return Some(Input::Guess(guess));
	}
}