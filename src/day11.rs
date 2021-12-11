use crate::error;

use std::collections::HashSet;

#[derive(PartialEq, Debug)]
pub struct GameState {
    pub grid: Vec<Vec<u64>>,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Flash {
    x: i32,
    y: i32,
}

impl std::str::FromStr for GameState {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<u64>> = s
            .lines()
            .filter(|line| !line.trim_start().trim_end().is_empty())
            .map(|line| line.chars().filter(|&c| c != ' ').map(|c| c.to_digit(10).unwrap() as u64).collect())
            .collect();
        Ok(GameState { grid })
    }
}

pub struct SimulationResult {
    pub game_state: GameState,
    pub total_flashes: usize,
    pub mega_flashes: Vec<usize>,
}

impl GameState {
    pub fn width(&self) -> i32 {
        self.grid[0].len() as i32
    }

    pub fn height(&self) -> i32 {
        self.grid.len() as i32
    }

    pub fn simulate(&self, num_steps: usize) -> SimulationResult {
        let mut game_state = GameState { grid: self.grid.clone() };
        let mut total_flashes = 0;
        let mut mega_flashes: Vec<usize> = Vec::new();

        for iteration in 0..num_steps {
            let flashes = game_state.simulate_one_step();
            total_flashes += flashes;
            if flashes as i32 == (self.width() * self.height()) {
                mega_flashes.push(iteration + 1);
            }
        }

        SimulationResult {
            game_state,
            total_flashes,
            mega_flashes,
        }
    }

    fn _dump(&self) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                print!("{:3} ", self.grid[x as usize][y as usize]);
            }
            println!();
        }
        println!();
    }

    fn should_flash(&self, x: i32, y: i32) -> bool {
        self.grid[x as usize][y as usize] > 9
    }

    fn increase_by_one_unless_oob(&mut self, x: i32, y: i32) {
        let width = self.width();
        let height = self.height();
        if x < 0 || x >= width || y < 0 || y >= height {
            return;
        }
        self.grid[x as usize][y as usize] += 1
    }

    fn apply_flash(&mut self, flash: &Flash) {
        let x = flash.x;
        let y = flash.y;
        self.increase_by_one_unless_oob(x, y - 1);
        self.increase_by_one_unless_oob(x + 1, y - 1);
        self.increase_by_one_unless_oob(x + 1, y);
        self.increase_by_one_unless_oob(x + 1, y + 1);
        self.increase_by_one_unless_oob(x, y + 1);
        self.increase_by_one_unless_oob(x - 1, y + 1);
        self.increase_by_one_unless_oob(x - 1, y);
        self.increase_by_one_unless_oob(x - 1, y - 1);
    }

    fn simulate_one_step(&mut self) -> usize {
        for y in 0..self.height() {
            for x in 0..self.width() {
                self.increase_by_one_unless_oob(x, y);
            }
        }

        let mut all_flashes: HashSet<Flash> = HashSet::new();

        loop {
            let mut new_flashes: HashSet<Flash> = HashSet::new();

            for y in 0..self.height() {
                for x in 0..self.width() {
                    if self.should_flash(x, y) {
                        let flash = Flash { x, y };
                        if !all_flashes.contains(&flash) {
                            new_flashes.insert(flash.clone());
                            all_flashes.insert(flash);
                        }
                    }
                }
            }

            for flash in &new_flashes {
                self.apply_flash(flash);
            }

            if new_flashes.is_empty() {
                break;
            }
        }

        for flash in &all_flashes {
            self.grid[flash.x as usize][flash.y as usize] = 0;
        }

        all_flashes.len() as usize
    }
}

#[test]
fn test_day11_mini() -> Result<(), error::Error> {
    let input = r#"
        11111
        19991
        19191
        19991
        11111"#;
    let initial_state: GameState = input.parse()?;

    assert_eq!(initial_state.width(), 5);
    assert_eq!(initial_state.height(), 5);

    assert_eq!(
        initial_state.simulate(1).game_state,
        r#"
        34543
        40004
        50005
        40004
        34543"#
            .parse()?
    );

    Ok(())
}

#[test]
fn test_day11() -> Result<(), error::Error> {
    let initial_state: GameState = r#"
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"#
    .parse()?;
    assert_eq!(initial_state.width(), 10);
    assert_eq!(initial_state.height(), 10);

    let expected_state: GameState = r#"
0397666866
0749766918
0053976933
0004297822
0004229892
0053222877
0532222966
9322228966
7922286866
6789998766"#
        .parse()?;
    let result = initial_state.simulate(100);
    assert_eq!(result.game_state, expected_state);
    assert_eq!(result.total_flashes, 1656);
    let result = initial_state.simulate(195);
    assert_eq!(result.mega_flashes.first().unwrap().to_owned(), 195);

    let initial_state: GameState = std::fs::read_to_string("input_day11")?.parse()?;
    let result = initial_state.simulate(100);
    assert_eq!(result.total_flashes, 1642);
    let result = initial_state.simulate(320);
    assert_eq!(result.mega_flashes.first().unwrap().to_owned(), 320);
    Ok(())
}
