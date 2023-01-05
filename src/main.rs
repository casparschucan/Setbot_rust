extern crate serenity;
extern crate rand;

use svg::Document;

use svg::node::element::Rectangle;
use svg::node::element::SVG;
use crate::set::set::parse_input;
use crate::set::set::new_game;
use crate::set::set::Input;
use crate::set::set::add_cards;

pub mod set;
fn main() {
    
    let mut cards: Vec<SVG> = vec![];
    for k in 0..3 {
        for j in 0..4 {
            let rect: Rectangle = Rectangle::new().set("x", 0_i32)
                .set("y", 0_i32)
                .set("stroke", "black")
                .set("stroke-width", 3_i32)
                .set("fill", "white")
                .set("height", 100_i32)
                .set("width", 160_i32);
            let card = SVG::new()
                .set("x", k * 160_i32)
                .set("y", j * 100_i32)
                .set("height", 100_i32)
                .set("width", 160_i32)
                .add(rect);
            cards.push(card);
        }
    } 
    
    let mut document = Document::new()
        .set("height", 500_i32)
        .set("width", 500_i32);
    
    for card in cards.clone() {
        document = document.add(card);
    }
    
    svg::save("image.svg", &document).unwrap();
    let mut game = new_game();
    while !game.ended {
        let basic_guess: Option<Input> = parse_input();
        match basic_guess {
            Some(x) => {
                match x {
                    Input::Guess(y) => game.guess(y[0], y[1], y[2]),
                    Input::Command(_y) => add_cards(),
                }
            }
            None =>  {
                println!("invalid guess format. Couldn't parse");
            }
        }
    }
}
