use crate::error;

use std::collections::HashMap;

fn get_two_chars_from_pair(pair: &str) -> Option<(char, char)> {
    let mut chars = pair.chars();
    let char1 = chars.next();
    let char2 = chars.next();
    let char3 = chars.next();
    if char1.is_none() || char2.is_none() || char3.is_some() {
        return None;
    }
    Some((char1.unwrap(), char2.unwrap()))
}

fn template_to_pair_counter(s: &str) -> HashMap<String, usize> {
    let mut pair_counter = HashMap::new();
    for i in 0..s.len() - 1 {
        let from = &s[i..=(i + 1)];
        *pair_counter.entry(from.to_string()).or_default() += 1
    }
    pair_counter
}

pub struct Game {
    pub template: String,
    pub instructions: HashMap<String, char>,
}

impl std::str::FromStr for Game {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().filter(|t| !t.trim_start().trim_end().is_empty());
        Ok(Game {
            template: lines.next().unwrap().to_string(),
            instructions: lines.fold(HashMap::new(), |mut acc, l| {
                let mut tokens = l.split(&[' ', '-', '>'][..]).filter(|t| !t.trim_start().trim_end().is_empty());
                let from = tokens.next().unwrap();
                let to = tokens.next().unwrap().chars().next().unwrap();
                acc.entry(from.to_string()).or_insert(to);
                acc
            }),
        })
    }
}

pub struct GameResult {
    pair_counter: HashMap<String, usize>,
    template: String,
}

impl GameResult {
    pub fn score(&self) -> usize {
        let mut char_counter: HashMap<char, usize> = HashMap::new();
        for (k, v) in &self.pair_counter {
            let mut chars = k.chars();
            let char1 = chars.next().unwrap();
            let char2 = chars.next().unwrap();
            *char_counter.entry(char1).or_default() += v;
            *char_counter.entry(char2).or_default() += v;
        }

        let first_template_char = self.template.chars().next().unwrap();
        let last_template_char = self.template.chars().last().unwrap();

        for (&k, v) in char_counter.iter_mut() {
            if k == first_template_char {
                *v -= 1;
            }
            if k == last_template_char {
                *v -= 1;
            }
            *v /= 2;

            if k == first_template_char {
                *v += 1;
            }
            if k == last_template_char {
                *v += 1;
            }
        }

        let numbers: Vec<usize> = char_counter.into_iter().map(|(_, y)| y).collect();
        let max = numbers.iter().max().unwrap().to_owned();
        let min = numbers.iter().min().unwrap().to_owned();
        max - min
    }
}

impl Game {
    fn generate_two_pairs_from_pair(&self, from: &str) -> (String, String) {
        let to = self.instructions.get(from).unwrap();
        let (char1, char2) = get_two_chars_from_pair(from).unwrap();

        (format!("{}{}", char1, to), format!("{}{}", to, char2))
    }

    pub fn step(&self, times: usize) -> GameResult {
        let mut pair_counter_current = template_to_pair_counter(&self.template);

        for _iteration in 0..times {
            let mut pair_counter_next: HashMap<String, usize> = HashMap::new();

            for (k, v) in &pair_counter_current {
                let (pair1, pair2) = self.generate_two_pairs_from_pair(k);
                *pair_counter_next.entry(pair1).or_default() += v;
                *pair_counter_next.entry(pair2).or_default() += v;
            }

            pair_counter_current = pair_counter_next;
        }

        GameResult {
            pair_counter: pair_counter_current,
            template: self.template.to_string(),
        }
    }
}

#[test]
fn test_day14() -> Result<(), error::Error> {
    let input = r#"
NN

CC -> N
NN -> C
NC -> C
CN -> N
"#;
    let game: Game = input.parse()?;
    assert_eq!(game.template, "NN");
    assert_eq!(game.instructions.len(), 4);
    assert_eq!(game.instructions.get("CC").unwrap(), &'N');
    assert_eq!(game.step(0).score(), 0);
    assert_eq!(game.step(1).score(), 1);
    assert_eq!(game.step(2).score(), 1);

    let input = r#"
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;
    let game: Game = input.parse()?;
    assert_eq!(game.template, "NNCB");
    assert_eq!(game.instructions.len(), 16);
    assert_eq!(game.instructions.get("NB").unwrap(), &'B');
    assert_eq!(game.step(1).score(), 1);
    assert_eq!(game.step(10).score(), 1588);
    assert_eq!(game.step(40).score(), 2188189693529);

    let game: Game = std::fs::read_to_string("input_day14")?.parse()?;
    assert_eq!(game.step(10).score(), 3259);
    assert_eq!(game.step(40).score(), 3459174981021);

    Ok(())
}
