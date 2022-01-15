use crate::error;

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct DiceOutcome {
    value: usize,
    weight: usize,
}

pub trait Die {
    fn roll_three(&mut self) -> Vec<DiceOutcome>;
    fn num_rolls(&self) -> usize;
}

#[derive(Default)]
pub struct PracticeDie {
    num_rolls: usize,
}

#[derive(Default)]
pub struct DiracDie {
    num_rolls: usize,
}

impl PracticeDie {
    fn roll(&mut self) -> usize {
        self.num_rolls += 1;
        ((self.num_rolls() - 1) % 100) + 1
    }
}

impl Die for PracticeDie {
    fn roll_three(&mut self) -> Vec<DiceOutcome> {
        vec![DiceOutcome {
            value: self.roll() + self.roll() + self.roll(),
            weight: 1,
        }]
    }

    fn num_rolls(&self) -> usize {
        self.num_rolls
    }
}

impl Die for DiracDie {
    fn roll_three(&mut self) -> Vec<DiceOutcome> {
        self.num_rolls += 3;
        vec![
            DiceOutcome { value: 3, weight: 1 },
            DiceOutcome { value: 4, weight: 3 },
            DiceOutcome { value: 5, weight: 6 },
            DiceOutcome { value: 6, weight: 7 },
            DiceOutcome { value: 7, weight: 6 },
            DiceOutcome { value: 8, weight: 3 },
            DiceOutcome { value: 9, weight: 1 },
        ]
    }

    fn num_rolls(&self) -> usize {
        self.num_rolls
    }
}

pub struct Game {
    player1_starting_position: usize,
    player2_starting_position: usize,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct GameState {
    p1_pos: usize,
    p2_pos: usize,
    p1_score: usize,
    p2_score: usize,
    next_player: usize,
}

impl GameState {
    fn new(p1_pos: usize, p2_pos: usize) -> Self {
        Self {
            p1_pos,
            p2_pos,
            p1_score: 0,
            p2_score: 0,
            next_player: 1,
        }
    }

    fn is_end_state(&self, winning_score: usize) -> bool {
        self.p1_score >= winning_score || self.p2_score >= winning_score
    }

    fn play(&self, value: usize) -> Self {
        let mut new_state = *self;
        match new_state.next_player {
            1 => {
                new_state.move_p1(value);
                new_state.p1_score += new_state.p1_pos;
                new_state.next_player = 2;
            }
            2 => {
                new_state.move_p2(value);
                new_state.p2_score += new_state.p2_pos;
                new_state.next_player = 1;
            }
            _ => panic!("no such player: {}", new_state.next_player),
        }
        new_state
    }

    fn move_player(from: usize, steps: usize) -> usize {
        let new_pos = from + steps;
        ((new_pos - 1) % 10) + 1
    }

    fn move_p1(&mut self, steps: usize) {
        self.p1_pos = GameState::move_player(self.p1_pos, steps);
    }

    fn move_p2(&mut self, steps: usize) {
        self.p2_pos = GameState::move_player(self.p2_pos, steps);
    }
}

pub struct GameResult {
    pub states: HashMap<GameState, usize>,
    pub num_die_rolls: usize,
}

impl GameResult {
    pub fn calc_part1(&self) -> usize {
        if self.states.len() != 1 {
            panic!("invalid game state for part1 (1)");
        }
        let state = self.states.iter().next().unwrap();
        if state.1 != &1 {
            panic!("invalid game state for part1 (2)");
        }
        let state = state.0;
        usize::min(state.p1_score, state.p2_score) * self.num_die_rolls
    }

    pub fn calc_part2(&self) -> usize {
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        for (state, num) in self.states.iter() {
            if state.p1_score > state.p2_score {
                p1_wins += num;
            } else {
                p2_wins += num;
            }
        }
        usize::max(p1_wins, p2_wins)
    }
}

impl Game {
    pub fn play(&self, die: &mut impl Die, winning_score: usize) -> GameResult {
        let initial_state = GameState::new(self.player1_starting_position, self.player2_starting_position);

        let mut states: HashMap<GameState, usize> = HashMap::new();
        let mut end_states: HashMap<GameState, usize> = HashMap::new();

        *states.entry(initial_state).or_default() += 1;

        loop {
            let mut new_states: HashMap<GameState, usize> = HashMap::new();

            for (state, &amount) in states.iter() {
                let dice_outcomes = die.roll_three();
                for outcome in dice_outcomes.iter() {
                    let new_state = state.play(outcome.value);
                    if new_state.is_end_state(winning_score) {
                        *end_states.entry(new_state).or_default() += amount * outcome.weight;
                    } else {
                        *new_states.entry(new_state).or_default() += amount * outcome.weight;
                    }
                }
            }

            if new_states.is_empty() {
                break;
            }

            states = new_states;
        }

        GameResult {
            states: end_states,
            num_die_rolls: die.num_rolls(),
        }
    }
}

impl std::str::FromStr for Game {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().map(|l| l.trim_start().trim_end()).filter(|l| !l.is_empty());
        let p1_start = lines.next().unwrap().split(':').nth(1).unwrap().trim_start().trim_end();
        let p2_start = lines.next().unwrap().split(':').nth(1).unwrap().trim_start().trim_end();
        Ok(Self {
            player1_starting_position: p1_start.parse()?,
            player2_starting_position: p2_start.parse()?,
        })
    }
}

#[test]
fn test_i64() {
    assert!(444356092776315 < i64::MAX);
    assert!(341960390180808 < i64::MAX);
}

#[test]
fn test_die() -> Result<(), error::Error> {
    let mut die = PracticeDie::default();

    assert_eq!(die.roll_three(), vec![DiceOutcome { value: 1 + 2 + 3, weight: 1 }]);
    assert_eq!(die.roll_three(), vec![DiceOutcome { value: 4 + 5 + 6, weight: 1 }]);
    assert_eq!(die.roll_three(), vec![DiceOutcome { value: 7 + 8 + 9, weight: 1 }]);
    assert_eq!(die.roll_three(), vec![DiceOutcome { value: 10 + 11 + 12, weight: 1 }]);
    for _ in 0..26 {
        die.roll_three();
    }
    assert_eq!(die.roll_three(), vec![DiceOutcome { value: 91 + 92 + 93, weight: 1 }]);

    let mut die = DiracDie::default();
    assert_eq!(die.roll_three().iter().map(|o| o.weight).sum::<usize>(), 27);
    assert_eq!(die.roll_three().iter().map(|o| o.value).sum::<usize>(), 42);

    Ok(())
}

#[test]
fn test_board() -> Result<(), error::Error> {
    let mut state = GameState::new(4, 8);
    state = state.play(1 + 2 + 3);
    state = state.play(4 + 5 + 6);
    state = state.play(7 + 8 + 9);
    state = state.play(10 + 11 + 12);
    assert_eq!(state.p1_score, 14);
    assert_eq!(state.p2_score, 9);
    Ok(())
}

#[test]
fn test_day21() -> Result<(), error::Error> {
    let input = r#"
Player 1 starting position: 4
Player 2 starting position: 8
"#;
    let game: Game = input.parse()?;
    assert_eq!(game.player1_starting_position, 4);
    assert_eq!(game.player2_starting_position, 8);

    let mut die = PracticeDie::default();
    let result = game.play(&mut die, 1000);
    //assert_eq!(result.num_die_rolls, 993);
    assert_eq!(result.calc_part1(), 739785);

    let mut die = DiracDie::default();
    let result = game.play(&mut die, 21);
    assert_eq!(result.calc_part2(), 444356092776315);

    let game: Game = std::fs::read_to_string("input_day21")?.parse()?;
    assert_eq!(game.player1_starting_position, 4);
    assert_eq!(game.player2_starting_position, 10);

    let mut die = PracticeDie::default();
    let result = game.play(&mut die, 1000);
    assert_eq!(result.calc_part1(), 855624);

    let mut die = DiracDie::default();
    let result = game.play(&mut die, 21);
    assert_eq!(result.calc_part2(), 187451244607486);

    Ok(())
}
