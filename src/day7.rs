use crate::error;

fn cost_distance_constant(v1: u64, v2: u64) -> u64 {
    (v1 as i32 - v2 as i32).abs() as u64
}

fn cost_distance_increasing(v1: u64, v2: u64) -> u64 {
    // https://en.wikipedia.org/wiki/Triangular_number
    let distance = (v1 as i32 - v2 as i32).abs() as u64;
    (distance * (distance + 1)) / 2
}

pub enum CrabGameMode {
    ConstantCost,
    IncreasingCost,
}

impl CrabGameMode {
    pub fn distance_cost(&self, v1: u64, v2: u64) -> u64 {
        match self {
            CrabGameMode::ConstantCost => cost_distance_constant(v1, v2),
            CrabGameMode::IncreasingCost => cost_distance_increasing(v1, v2),
        }
    }
}

pub struct CrabGame {
    pub positions: Vec<u64>,
}

#[derive(PartialEq, Debug)]
pub struct CrabGameResult {
    cost: u64,
    position: usize,
}

impl CrabGame {
    pub fn cheapest(&self, mode: CrabGameMode) -> CrabGameResult {
        let cheapest = (0..=self.positions.iter().max().unwrap().to_owned())
            .map(|destination_position| self.positions.iter().map(|&p| mode.distance_cost(p, destination_position as u64)).sum())
            .enumerate()
            .min_by(|lhs: &(usize, u64), rhs: &(usize, u64)| lhs.1.cmp(&rhs.1))
            .unwrap();

        CrabGameResult {
            cost: cheapest.1,
            position: cheapest.0,
        }
    }
}

impl std::str::FromStr for CrabGame {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<u64> = s
            .split(&[',', '\n', ' '][..])
            .filter(|token| !token.trim_start().trim_end().is_empty())
            .map(|token| token.parse().unwrap())
            .collect();
        Ok(CrabGame { positions: values })
    }
}

#[test]
fn test_distance_cost() {
    assert_eq!(cost_distance_constant(0, 0), 0);
    assert_eq!(cost_distance_constant(0, 10), 10);
    assert_eq!(cost_distance_constant(10, 0), 10);

    assert_eq!(cost_distance_increasing(0, 0), 0);
    assert_eq!(cost_distance_increasing(0, 1), 1);
    assert_eq!(cost_distance_increasing(0, 2), 3);
    assert_eq!(cost_distance_increasing(3, 0), 6);
    assert_eq!(cost_distance_increasing(1, 5), 10);
    assert_eq!(cost_distance_increasing(14, 5), 45);
    assert_eq!(cost_distance_increasing(16, 5), 66);
}

#[test]
fn test_crab_game() -> Result<(), error::Error> {
    let input = "16,1,2,0,4,2,7,1,2,14";
    let game: CrabGame = input.parse()?;

    assert_eq!(game.positions.len(), 10);
    assert_eq!(game.cheapest(CrabGameMode::ConstantCost), CrabGameResult { cost: 37, position: 2 });
    assert_eq!(game.cheapest(CrabGameMode::IncreasingCost), CrabGameResult { cost: 168, position: 5 });

    let input = std::fs::read_to_string("input_day7")?;
    let game: CrabGame = input.parse()?;

    assert_eq!(game.positions.len(), 1000);
    assert_eq!(game.cheapest(CrabGameMode::ConstantCost).cost, 331067);
    assert_eq!(game.cheapest(CrabGameMode::IncreasingCost).cost, 92881128);

    Ok(())
}

#[test]
fn test_david() -> Result<(), error::Error> {
    let input = "0,1,2,2,3,3,3,4,6,6,6";
    let game: CrabGame = input.parse()?;
    assert_eq!(game.positions.len(), 11);
    assert_eq!(game.cheapest(CrabGameMode::IncreasingCost), CrabGameResult { cost: 30, position: 3 });
    Ok(())
}
