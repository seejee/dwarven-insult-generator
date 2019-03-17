use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::fs;

enum InsultPart {
    Opener,
    Middle,
    Closer,
}

#[derive(Clone)]
pub struct Config {
    openers: Vec<String>,
    middles: Vec<String>,
    closers: Vec<String>,
}

impl<'a> Config {
    pub fn from_files() -> Config {
        Config {
            openers: parse_words("./src/data/openers.txt"),
            middles: parse_words("./src/data/middles.txt"),
            closers: parse_words("./src/data/closers.txt"),
        }
    }

    fn random_part(&self, part: InsultPart) -> &str {
        let words = match part {
            InsultPart::Opener => &self.openers,
            InsultPart::Middle => &self.middles,
            _ => &self.closers,
        };

        let mut rng = thread_rng();
        words.choose(&mut rng).unwrap()
    }
}

fn parse_words(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).unwrap();
    let lines = contents.lines().map(|l| l.to_string()).collect();

    lines
}

fn parts_for_roll(roll: u8) -> Vec<InsultPart> {
    match roll {
        1..=6 => vec![InsultPart::Opener, InsultPart::Middle, InsultPart::Closer],
        7..=8 => vec![
            InsultPart::Opener,
            InsultPart::Opener,
            InsultPart::Middle,
            InsultPart::Closer,
        ],
        9..=10 => vec![
            InsultPart::Opener,
            InsultPart::Middle,
            InsultPart::Middle,
            InsultPart::Closer,
        ],
        11..=12 => vec![
            InsultPart::Opener,
            InsultPart::Opener,
            InsultPart::Middle,
            InsultPart::Middle,
            InsultPart::Closer,
        ],
        13..=14 => vec![
            InsultPart::Opener,
            InsultPart::Middle,
            InsultPart::Opener,
            InsultPart::Closer,
        ],
        15..=16 => vec![
            InsultPart::Opener,
            InsultPart::Middle,
            InsultPart::Opener,
            InsultPart::Middle,
            InsultPart::Closer,
        ],
        17..=18 => vec![
            InsultPart::Opener,
            InsultPart::Middle,
            InsultPart::Middle,
            InsultPart::Opener,
            InsultPart::Closer,
        ],
        _ => vec![InsultPart::Opener, InsultPart::Closer],
    }
}

fn roll_d20() -> u8 {
    let mut rng = thread_rng();
    let roll: u8 = rng.gen_range(1, 21);
    roll
}

fn capitalize_first(s: String) -> String {
    let mut v: Vec<char> = s.chars().collect();
    v[0] = v[0].to_uppercase().nth(0).unwrap();
    v.into_iter().collect()
}

pub fn generate_insult(config: &Config) -> String {
    let roll = roll_d20();
    let parts = parts_for_roll(roll);

    let words: Vec<_> = parts
        .into_iter()
        .map(|part| config.random_part(part))
        .collect();

    capitalize_first(format!("{}!", words.join(", ")))
}
