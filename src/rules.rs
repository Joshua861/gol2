use macroquad::rand::{self, gen_range, rand};
use serde::{Deserialize, Serialize};

use crate::{board::Board, config::Config};

macro_rules! rulestring {
    ($s: expr) => {{
        let bytes = $s.as_bytes();
        let mut i = 0;
        let mut survive = [false; 9];
        let mut spawn = [false; 9];
        let mut parsing_survive = true;

        while i < bytes.len() {
            match bytes[i] as char {
                '/' => {
                    parsing_survive = false;
                    i += 1;
                    continue;
                }
                c @ '0'..='8' => {
                    let idx = char_to_digit!(c);
                    if parsing_survive {
                        survive[idx] = true;
                    } else {
                        spawn[idx] = true;
                    }
                }
                _ => {}
            }
            i += 1;
        }

        Rule::Rulestring(Rulestring { survive, spawn })
    }};
}

macro_rules! char_to_digit {
    ($c: expr) => {
        match $c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _ => panic!("Invalid digit"),
        }
    };
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum Rule {
    Rulestring(Rulestring),
    Custom(usize),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Copy)]
pub struct Rulestring {
    pub survive: [bool; 9],
    pub spawn: [bool; 9],
}

pub const CONWAY: Rule = rulestring!("23/3");
pub const MAZE: Rule = rulestring!("12345/3");
pub const MAZE_MICE: Rule = rulestring!("12345/37");
pub const FALLING_STARS: Rule = Rule::Custom(0);

pub const RULES: &[(&str, Rule)] = &[
    ("Conway", CONWAY),
    ("Maze", MAZE),
    ("Maze with mice", MAZE_MICE),
    ("Falling stars", FALLING_STARS),
];

type CustomRule = fn(&mut Board, &Config);

pub const CUSTOM_RULES: &[CustomRule] = &[falling_stars];

fn falling_stars(board: &mut Board, config: &Config) {
    // {
    //     let s = (board.width() + board.height()) / 10;
    //
    //     for _ in 0..s {
    //         let y = gen_range(0., board.height() as f32) as isize;
    //         let x = gen_range(0., board.width() as f32) as isize;
    //
    //         board.set(x, y, true);
    //     }
    // }

    for y in 0..board.height() {
        for x in 0..board.width() {
            let (x, y) = (x as isize, y as isize);
            if rand::rand() % 40 == 0 {
                board.set(x, y, false);
            }
        }
    }

    let old = board.clone();

    for y in 0..board.height() {
        for x in 0..board.width() {
            let (x, y) = (x as isize, y as isize);

            if rand() % 2000 == 0 {
                board.set(x, y, true);
            }

            let t = old.get(x, y);

            if t.alive() {
                if !old.get(x, y + 1).alive() {
                    board.set(x, y + 1, true);
                    board.set(x, y, false);
                } else {
                    board.set(x, y, false);
                }
            }

            board.get_mut(x, y).update_heat(config);

            let t = board.get_mut(x, y);
            t.heat = t.heat.saturating_sub(10);
        }
    }
}
