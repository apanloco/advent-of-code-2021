use crate::error;

#[derive(Debug)]
pub struct Board {
    positions: Vec<Vec<u8>>,
}

impl std::str::FromStr for Board {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Board {
            positions: s
                .lines()
                .filter(|l| !l.trim_start().trim_end().is_empty())
                .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
                .collect(),
        })
    }
}

impl Board {
    pub fn lowest_total_risk(&self) -> i32 {
        let width = || self.positions[0].len() as i32;

        let height = || self.positions.len() as i32;

        let is_oob = |x, y| -> bool { x < 0 || x >= width() || y < 0 || y >= height() };

        let at = |x, y| self.positions[y as usize][x as usize] as i32;

        let cost_to = |x, y| {
            if is_oob(x, y) {
                return 999;
            }
            at(x, y)
        };

        pathfinding::directed::astar::astar(
            &(0, 0),
            |&(x, y)| vec![(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)].into_iter().map(|p| (p, cost_to(p.0, p.1))),
            |&(x, y)| (height() - y) + (width() - x),
            |&p| p.0 == width() - 1 && p.1 == height() - 1,
        )
        .unwrap()
        .1
    }

    pub fn lowest_total_risk_quintupled(&self) -> i32 {
        let width = || (self.positions[0].len() * 5) as i32;

        let height = || (self.positions.len() * 5) as i32;

        let is_oob = |x, y| -> bool { x < 0 || x >= width() || y < 0 || y >= height() };

        let at = |x, y| {
            let base_width = self.positions[0].len() as i32;
            let base_height = self.positions.len() as i32;
            let tile_x = x / base_width;
            let tile_y = y / base_height;
            let base_x = x % base_width;
            let base_y = y % base_height;

            let base_risk = self.positions[base_y as usize][base_x as usize] as i32;

            let mut new_risk = base_risk + tile_x + tile_y;

            if new_risk > 9 {
                new_risk -= 9;
            }

            new_risk
        };

        let cost_to = |x, y| {
            if is_oob(x, y) {
                return 999;
            }

            at(x, y)
        };

        pathfinding::directed::astar::astar(
            &(0, 0),
            |&(x, y)| vec![(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)].into_iter().map(|p| (p, cost_to(p.0, p.1))),
            |&(x, y)| (height() - y) + (width() - x),
            |&p| p.0 == width() - 1 && p.1 == height() - 1,
        )
        .unwrap()
        .1
    }
}

#[test]
fn test_day15() -> Result<(), error::Error> {
    let board: Board = r#"
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"#
    .parse()?;
    assert_eq!(board.lowest_total_risk(), 40);
    assert_eq!(board.lowest_total_risk_quintupled(), 315);

    let board: Board = std::fs::read_to_string("input_day15")?.parse()?;
    assert_eq!(board.lowest_total_risk(), 696);
    assert_eq!(board.lowest_total_risk_quintupled(), 2952);

    Ok(())
}
